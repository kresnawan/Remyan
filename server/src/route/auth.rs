use axum::{Extension, Router, routing::{get, post}};
use rand::RngExt;
use remyan_core::AppInstance;

use crate::{handler::auth::handle_login};

pub fn auth() -> Router {
    let router = Router::new()
        .route("/login", post(handle_login))
        .route("/id", get(|Extension(app): Extension<AppInstance>| async move {
            let id: u32;
            let mut app_instance = app.lock().await;

            loop {
                let try_id: u32 = rand::rng().random();
                if app_instance.players.contains_key(&try_id) {
                    continue;
                } else {
                    id = try_id;
                    break;
                }
            }

            app_instance.register_new_player(id).unwrap();


            format!("{}", id)
        }));

    Router::new().nest("/auth", router)
}
