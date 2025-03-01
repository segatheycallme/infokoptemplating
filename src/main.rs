use std::error::Error;

use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, response::IntoResponse, routing::get, serve};
use infokoptemplating::Weather;
use reqwest::Client;
use scraper::{Html, Selector};
use tokio::io::AsyncReadExt;
use tower_http::services::ServeFile;

#[allow(dead_code)]
const TAILWINDCSS_PATH: &str = "http://localhost:3000/style";

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
#[allow(dead_code)]
struct Hai<'a> {
    items: Vec<&'a str>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route_service("/style", ServeFile::new("templates/output.css"))
        .route("/", get(hi))
        .with_state(Client::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    serve(listener, app).await?;

    Ok(())
}

// GET http://localhost:3000
async fn hi(State(_client): State<Client>) -> impl IntoResponse {
    let mut body = String::new();
    tokio::fs::File::open(std::path::Path::new("./infokop.html"))
        .await
        .unwrap()
        .read_to_string(&mut body)
        .await
        .unwrap();
    // let body = _client
    //     .get("https://www.infokop.net/info/ski-info.html")
    //     .header("User-Agent", "insanely dumb bypass")
    //     .send()
    //     .await
    //     .unwrap()
    //     .text()
    //     .await
    //     .unwrap();

    let html = Html::parse_document(&body);
    let mut el_iter = html
        .select(&Selector::parse("table.contentpaneopen").unwrap())
        .nth(1)
        .unwrap()
        .descendent_elements()
        .nth(3)
        .unwrap()
        .child_elements();
    let weather = Weather::from_html_element(el_iter.nth(2).unwrap()).unwrap();
    // let piste = el_iter.next().unwrap();
    format!("{:#?}", weather)
}
