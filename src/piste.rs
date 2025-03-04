use std::collections::HashMap;

use scraper::{ElementRef, Selector};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Piste {
    pub day_lifts: Vec<Lift>,
    pub night_lifts: Vec<Lift>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Lift {
    pub online: bool,
    pub slope_name: String,
    pub skiers_per_hour: u16,
    pub length: u16,
    pub height: u16,
    pub duration: f32,
    pub slopes: Option<HashMap<String, bool>>,
    pub paths: Option<HashMap<String, bool>>,
    pub opening_time: String,
    pub closing_time: String,
}

impl Piste {
    pub fn from_html_element(el: ElementRef) -> Option<Self> {
        let mut day_lifts = vec![];
        let mut night_lifts = vec![];
        let day_selector = Selector::parse("style + tr").unwrap();
        let night_selector = Selector::parse("tr + tr").unwrap();

        for maybe_lift in el.select(&day_selector) {
            if let Some(lift) = Lift::from_html_element(maybe_lift) {
                day_lifts.push(lift);
            }
        }
        for maybe_lift in el.select(&night_selector).skip(2).take(3) {
            if let Some(lift) = Lift::from_html_element(maybe_lift) {
                night_lifts.push(lift);
            }
        }

        Some(Piste {
            day_lifts,
            night_lifts,
        })
    }
}

// GET http://localhost:3000
impl Lift {
    fn from_html_element(el: ElementRef) -> Option<Self> {
        let mut tds = el.child_elements();

        let first_td = tds.next()?;
        let online = first_td
            .child_elements()
            .next()?
            .attr("class")?
            .contains("clrGreen");
        let slope_name = first_td.text().nth(2)?.trim().to_string();
        let skiers_per_hour = tds
            .nth(1)?
            .text()
            .last()?
            .trim()
            .parse()
            .unwrap_or_default();
        let length = tds.next()?.text().last()?.trim().parse().ok()?;
        let height = tds.next()?.text().last()?.trim().parse().ok()?;
        let duration = tds
            .next()?
            .text()
            .last()?
            .trim()
            .replace(',', ".")
            .parse()
            .ok()?;

        let dry = |iter: ElementRef| {
            let mut map = HashMap::new();
            for item in iter.child_elements() {
                let online = item.attr("class")?.contains("clrGreen");
                map.insert(item.text().next()?.to_string(), online);
            }
            Some(map)
        };

        let mut peak = tds.next()?;
        let slopes = dry(peak);

        peak = tds.next().unwrap_or(peak);
        let paths = dry(peak);

        peak = tds.next().unwrap_or(peak);
        let mut times = peak.text();
        let opening_time = times.next()?.trim().to_string();
        let closing_time = times.next()?.trim().to_string();

        Some(Lift {
            online,
            slope_name,
            skiers_per_hour,
            length,
            height,
            duration,
            slopes,
            paths,
            opening_time,
            closing_time,
        })
    }
}
