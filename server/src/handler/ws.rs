use std::sync::Arc;

use axum::{
    Extension,
    extract::{Query, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use remyan_core::AppInstance;
use tokio::sync::Mutex;

use crate::{
    router::{RoomIdQuery, Server},
    ws::socket_handler::handle_socket,
};

pub async fn handle_connect(
    Extension(game_app): Extension<AppInstance>,
    Extension(server): Extension<Arc<Mutex<Server>>>,
    jar: CookieJar,
    Query(params): Query<RoomIdQuery>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let player_id_str = jar
        .get("id")
        .map(|cookie| cookie.value().to_string())
        .expect("Cookie tidak ditemukan");

    let player_id: u32 = player_id_str.parse().unwrap();
    let room_id: u64 = params.room_id.parse().unwrap();

    {
        let instance = game_app.lock().await;
        let room = instance.room_manager.rooms.get(&room_id);
        if let None = room {
            return (StatusCode::NOT_FOUND).into_response();
        }

        if let None = room.unwrap().players.get(&player_id) {
            return (StatusCode::NOT_FOUND).into_response();
        }
    }

    ws.on_upgrade(move |socket| handle_socket(socket, game_app, server, player_id, room_id))
}
