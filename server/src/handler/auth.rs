use axum::{Json, response::IntoResponse};
use axum_extra::extract::{CookieJar, cookie::Cookie};

use crate::router::UserPayload;

pub async fn handle_login(
    jar: CookieJar, 
    Json(body): Json<UserPayload>
) -> impl IntoResponse {
    let cookie = Cookie::build(("id", body.id))
        .path("/")
        .http_only(true)
        .build();

    let jar = jar.add(cookie);

    (jar, "Cookie telah dibuat")
}
