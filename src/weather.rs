use regex::Regex;
use scraper::{ElementRef, Selector};

#[derive(Debug)]
pub struct Weather {
    pub temperature: f32,
    pub humidity: u16,
    pub snow: u16,
    pub visual: String,
    pub visual_img: String,
    pub sunrise: String,
    pub sunset: String,
    pub temperature_pancic: u16,
    pub temperature_feel: u16,
    pub wind_average: f32,
    pub wind_max: f32,
    pub wind_direction: u16,
    pub pressure: f32,
    pub precipitation: f32,
    pub uv: f32,
    pub snow_piste: (u16, u16),
    pub measured_at: u16,
}

// GET http://localhost:3000
impl Weather {
    pub fn from_html_element(el: ElementRef) -> Option<Self> {
        let thead_selector = Selector::parse("thead > tr > td").unwrap();
        let mut thead_cells = el.select(&thead_selector);

        let visual_img = thead_cells
            .nth(5)?
            .child_elements()
            .next()?
            .attr("src")?
            .trim()
            .to_string();
        let temperature = thead_cells
            .next()?
            .text()
            .next()?
            .trim()
            .replace(',', ".")
            .parse()
            .ok()?;
        let humidity = thead_cells.nth(1)?.text().next()?.trim().parse().ok()?;
        let snow = thead_cells.next()?.text().next()?.trim().parse().ok()?;

        let tbody_selector = Selector::parse("tbody > tr > td").unwrap();
        let mut tbody_cells = el.select(&tbody_selector);

        let mut iter = tbody_cells.next()?.text();
        let visual = iter.next()?.trim().to_string();
        let sunrise = iter.nth(1)?.trim().trim_start_matches('↑').to_string();
        let sunset = iter.next()?.trim().trim_start_matches('↓').to_string();

        iter = tbody_cells.next()?.text();
        let temperature_pancic = iter.nth(3)?.trim().trim_end_matches('°').parse().ok()?;
        let temperature_feel = iter.nth(1)?.trim().trim_end_matches('°').parse().ok()?;

        iter = tbody_cells.next()?.text();
        let wind = iter.nth(2)?.trim();
        let wind_regex = Regex::new(r"(\d+,?\d?)").unwrap();
        let wind_average = wind_regex
            .find(wind)?
            .as_str()
            .replace(',', ".")
            .parse()
            .ok()?;
        let wind_max = wind_regex
            .find(iter.nth(1)?.trim())?
            .as_str()
            .replace(',', ".")
            .parse()
            .ok()?;
        let wind_direction_regex = Regex::new(r"([A-Z]+)").unwrap();
        let wind_direction_str = wind_direction_regex.find(wind)?.as_str().to_string();
        let wind_direction = wind_direction_to_degree(&wind_direction_str, None)?;

        iter = tbody_cells.next()?.text();
        let pressure = iter
            .next()?
            .trim()
            .trim_start_matches("pritisak ")
            .trim_end_matches(" hPa")
            .replace(',', ".")
            .parse::<f32>()
            .ok()?
            / 10.0;
        let percipitation: f32 = iter
            .next()?
            .trim()
            .trim_start_matches("padavine ")
            .trim_end_matches(" mm")
            .parse()
            .ok()?;
        let uv: f32 = iter.next()?.trim().trim_start_matches("UV ").parse().ok()?;

        iter = tbody_cells.next()?.text();
        let intermediary = iter.next()?.split_whitespace().nth(1)?.split_once('-')?;
        let snow_piste = (intermediary.0.parse().ok()?, intermediary.1.parse().ok()?);
        let measured_at = iter
            .next()?
            .trim_start_matches("mereno u ")
            .trim_end_matches("h")
            .parse()
            .ok()?;

        Some(Weather {
            snow,
            humidity,
            temperature,
            visual,
            visual_img,
            sunrise,
            sunset,
            temperature_pancic,
            temperature_feel,
            wind_average,
            wind_max,
            wind_direction,
            pressure,
            precipitation: percipitation,
            uv,
            snow_piste,
            measured_at,
        })
    }
}

fn wind_direction_to_degree(raw: &str, curr: Option<u16>) -> Option<u16> {
    if raw.is_empty() {
        return curr;
    }
    let letter_value = match raw.chars().last()? {
        'N' => Some(0),
        'E' => Some(90),
        'S' => Some(180),
        'W' => Some(270),
        _ => None,
    }?;
    if let Some(current_value) = curr {
        return wind_direction_to_degree(
            raw.get(0..(raw.len() - 1))?,
            Some((current_value + letter_value) / 2),
        );
    }
    wind_direction_to_degree(raw.get(0..(raw.len() - 1))?, Some(letter_value))
}
