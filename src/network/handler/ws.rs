use axum::{Extension, extract::{Query, WebSocketUpgrade}, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{AppInstance, network::{router::RoomIdQuery, ws::socket_handler::handle_socket}};

pub async fn handle_connect(
    Extension(game_app): Extension<AppInstance>,
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
        let temp = match instance.room_manager.rooms.get(&room_id) {
            Some(a) => a,
            None => {
                return (StatusCode::NOT_FOUND).into_response();
            }
        };
        match temp.players.get(&player_id) {
            Some(_) => {}
            None => {
                return (StatusCode::NOT_FOUND).into_response();
            }
        };
    }

    ws.on_upgrade(move |socket| handle_socket(socket, game_app, player_id, room_id))
}
