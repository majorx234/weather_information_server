use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam_queue::ArrayQueue;
use tokio;
use weather_information::{
    config::Config,
    server_state::{ServerElements, ServerState},
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
        let mut counter = 0;
        while true {
            std::thread::sleep(sec);
            counter += 1;
            let _ = arc_aq.force_push(counter);
        }
    });
    let addr = config.get_host_socket_addr();

    let routes_all = Router::new()
        .route("/weather_information", get(handler_weather_information))
        .with_state(server_state);

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("failed to start server");
}

async fn handler_weather_information(State(server_state): State<ServerState>) -> impl IntoResponse {
    let temperature = server_state.lock().unwrap().get_image_data();
    println!("->> {:12} - 30°C", "HANDLER");
    Html(format!("{}°C - it's getting hot", temperature))
}

