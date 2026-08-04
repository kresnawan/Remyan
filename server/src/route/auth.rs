use axum::{
    Extension, Router,
    routing::{get, post},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use rand::RngExt;

use crate::{AppInstance, handler::auth::handle_login};

pub fn auth() -> Router {
    let router = Router::new().route("/login", post(handle_login)).route(
        "/id",
        get(
            |Extension(app): Extension<AppInstance>, jar: CookieJar| async move {
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

                let new_cookie = Cookie::build(("id", id.to_string()))
                    .path("/")
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::None);

                let updated_jar = jar.add(new_cookie);

                app_instance.register_new_player(id).unwrap();

                (updated_jar, format!("{}", id))
            },
        ),
    );

    Router::new().nest("/auth", router)
}
