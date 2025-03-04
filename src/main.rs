mod piste;
mod weather;

use crate::piste::Piste;
use crate::weather::Weather;

use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, response::IntoResponse, routing::get, serve};
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use tower_http::services::ServeFile;

const TAILWINDCSS_PATH: &str = "http://localhost:3000/style";

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
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
    let body = _client
        .get("https://www.infokop.net/info/ski-info.html")
        .header("User-Agent", "insanely dumb bypass")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    #[allow(unused_variables)]
    let (weather, piste) = html_helper(body);

    Hai {
        items: vec!["caoo"],
    }
}

fn html_helper(body: String) -> (Option<Weather>, Option<Piste>) {
    let html = Html::parse_document(&body);
    let mut el_iter = html
        .select(&Selector::parse("table.contentpaneopen").unwrap())
        .nth(1)
        .unwrap()
        .descendent_elements()
        .nth(3)
        .unwrap()
        .child_elements();
    let weather = Weather::from_html_element(el_iter.nth(2).unwrap());
    let piste = Piste::from_html_element(el_iter.next().unwrap());
    (weather, piste)
}
