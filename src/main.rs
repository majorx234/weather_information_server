use axum::{
    Error,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router, headers::Header, http::HeaderMap,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crossbeam_queue::ArrayQueue;
use tokio;
use weather_information::{
    config::Config,
    scrap_manager::start_scraping,
    server_state::{ServerElements, ServerState},
};

#[tokio::main]
async fn main() {
    let config = Config::new();

    let aq = ArrayQueue::new(2);
    let arc_aq = Arc::new(aq);
    let server_state = Arc::new(Mutex::new(ServerElements::new(arc_aq.clone())));

    start_scraping(arc_aq);
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

