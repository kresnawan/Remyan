use axum::{Router, routing::get};

use crate::{network::{handler::ws::handle_connect}};

pub fn ws() -> Router {
    let router = Router::new().route(
        "/connect",
        get(handle_connect),
    );

    Router::new().nest("/ws", router)
}
