use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;

use weather_information::{
    config::Config,
};

#[tokio::main]
async fn main() {
    let config = Config::new();
    let addr = config.get_host_socket_addr();

    let routes_all = Router::new()
        .route("/weather_information", get(handler_weather_information));

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("failed to start server");
}



async fn handler_weather_information() -> impl IntoResponse {
    println!("->> {:12} - 30°C", "HANDLER");
    Html("30°C - it's getting hot")
}

