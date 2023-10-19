use axum::{
    Error,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router, headers::Header, http::HeaderMap,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam_queue::ArrayQueue;
use tokio;
use weather_information::{
    config::Config,
    server_state::{ServerElements, ServerState},
    page_scraper::PageScraper,
};

#[tokio::main]
async fn main() {
    let config = Config::new();

    let aq = ArrayQueue::new(2);
    let arc_aq = Arc::new(aq);
    let server_state = Arc::new(Mutex::new(ServerElements::new(arc_aq.clone())));
    let scrapper_thread = thread::spawn(move || {
        let config = Config::new();
        let sec = config.get_scrap_frequency();
        let url_plain = config.get_weather_url();
        while true {
            std::thread::sleep(sec);
            let weather_page = PageScraper::new(&url_plain.to_string(), &config.get_selector(), config.get_data_index());
            if let Ok(temperatur_data) =
                weather_page.extract_temperature_from() {
                    let _ = arc_aq.force_push(temperatur_data);
                }
        }
    });
    let addr = config.get_host_socket_addr();

    let routes_all = Router::new()
        .route("/weather_information.png", get(handler_weather_information))
        .with_state(server_state);

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("failed to start server");
}

async fn handler_weather_information(State(server_state): State<ServerState>) -> impl IntoResponse {
    let temperature_bytes = server_state.lock().unwrap().get_image_data();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "image/png".parse().unwrap());

    (headers, temperature_bytes)
//    Html(format!("1°C - it's getting hot"))
}

