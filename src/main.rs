mod piste;
mod weather;

use crate::piste::Piste;
use crate::weather::Weather;

use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, response::IntoResponse, routing::get, serve};
use reqwest::{Client, ClientBuilder};
use scraper::{Html, Selector};
use std::error::Error;
use tower_http::{compression::CompressionLayer, services::ServeFile};

const TAILWINDCSS_PATH: &str = "/style";

#[derive(Template, WebTemplate)]
#[template(path = "app.html")]
struct App {
    piste: Piste,
    weather: Weather,
}

// GET http://localhost:3000/mermer
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route_service("/style", ServeFile::new("style.css"))
        .route("/", get(hi))
        .layer(CompressionLayer::new().br(true))
        .with_state(ClientBuilder::new().brotli(true).build()?);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    serve(listener, app).await?;

    Ok(())
}

// GET http://localhost:3000
async fn hi(State(_client): State<Client>) -> impl IntoResponse {
    let body = _client
        .get("https://www.infokop.net/info/ski-info.html")
        .header("User-Agent", "insanely dumb bypass")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let (weather, piste) = html_helper(body).unwrap();
    //
    // File::create(std::path::Path::new("weather.json"))
    //     .await
    //     .unwrap()
    //     .write_all(serde_json::to_string_pretty(&weather).unwrap().as_bytes())
    //     .await
    //     .unwrap();
    // let mut raw_json = String::new();
    // File::open(std::path::Path::new("weather.json"))
    //     .await
    //     .unwrap()
    //     .read_to_string(&mut raw_json)
    //     .await
    //     .unwrap();
    // let weather: Weather = serde_json::from_str(&raw_json).unwrap();
    App { piste, weather }
}

fn html_helper(body: String) -> Option<(Weather, Piste)> {
    let html = Html::parse_document(&body);
    let mut el_iter = html
        .select(&Selector::parse("table.contentpaneopen").unwrap())
        .nth(1)?
        .descendent_elements()
        .nth(3)?
        .child_elements();
    let weather = Weather::from_html_element(el_iter.nth(2)?)?;
    let piste = Piste::from_html_element(el_iter.next()?)?;
    Some((weather, piste))
}
