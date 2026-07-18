use std::sync::Arc;

use axum::{
    Extension,
    extract::{Query, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
};
use remyan_core::AppInstance;
use tokio::sync::Mutex;

use crate::{
    router::{RoomIdAndPlayerIdQuery, Server},
    ws::socket_handler::handle_socket,
};

pub async fn handle_connect(
    Extension(game_app): Extension<AppInstance>,
    Extension(server): Extension<Arc<Mutex<Server>>>,
    Query(params): Query<RoomIdAndPlayerIdQuery>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let player_id: u32;
    let player_id_str = params.player_id;

    if let Ok(value) = player_id_str.parse::<u32>() {
        player_id = value;
    } else {
        return (StatusCode::BAD_REQUEST).into_response();
    }

    let chars = params.room_id.as_bytes();
    let mut room_id = [0u8; 6];

    room_id.copy_from_slice(&chars[0..6]);

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
