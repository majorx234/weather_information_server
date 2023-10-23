use crate::server_state::ServerState;
use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Router,
    http::HeaderMap
};

pub fn get_route() -> Router<ServerState> {
    Router::new()
        .route("/weather_information.png", get(weather_information_image))
        .route("/weather_information.eink",get(weather_information_eink))
}

async fn weather_information_image(State(server_state): State<ServerState>) -> impl IntoResponse {
    let png_image = server_state.lock().unwrap().get_image_data();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "image/png".parse().unwrap());

    (headers, png_image)
}

async fn weather_information_eink(State(server_state): State<ServerState>) -> impl IntoResponse {
    let eink_image = server_state.lock().unwrap().get_eink_data();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/octet-stream".parse().unwrap());

    (headers, eink_image)
}
