use axum::{
    Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crossbeam_queue::ArrayQueue;
use tokio;
use weather_information::{
    config::Config,
    scrap_manager::start_scraping,
    server_state::ServerElements,
    routers::weather_information_image,
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
        .merge(weather_information_image::get_route())
        .with_state(server_state);

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("failed to start server");
}
