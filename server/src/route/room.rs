use axum::{
    Router,
    routing::{post},
};

use crate::handler::room::{handle_create_room, handle_join_room};

pub fn room() -> Router {
    let router = Router::new()
        .route(
            "/create",
            post(handle_create_room),
        )
        .route(
            "/join",
            post(handle_join_room),
        );

    Router::new().nest("/room", router)
}
