use axum::{Router, routing::post};

use crate::handler::auth::handle_login;

pub fn auth() -> Router {
    let router = Router::new().route("/login", post(handle_login));

    Router::new().nest("/auth", router)
}
