use axum::{Extension, Json, extract::Query, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use remyan_core::{AppInstance, RoomConfig};

use crate::{ServerInstance, router::RoomIdQuery, server_room::ServerRoom};

pub async fn handle_create_room(
    Extension(app): Extension<AppInstance>,
    Extension(server): Extension<ServerInstance>,
    jar: CookieJar,
    Json(config): Json<RoomConfig>,
) -> impl IntoResponse {
    let player_id_str = jar
        .get("id")
        .map(|cookie| cookie.value().to_string())
        .expect("Cookie tidak ditemukan");

    let player_id: u32 = player_id_str.parse().unwrap();
    let result;

    {
        let mut instance = app.lock().await;
        let mut server_instance = server.lock().await;
        
        result = match instance.create_room(player_id, config) {
            Ok(res) => res,
            Err(err) => {
                return (StatusCode::BAD_REQUEST, err);
            }
        };

        server_instance.rooms.insert(result, ServerRoom::new(result));
    }

    (
        StatusCode::OK,
        format!("Room berhasil dibuat, id: {}", result),
    )
}

pub async fn handle_join_room(
    Extension(app): Extension<AppInstance>,
    Query(params): Query<RoomIdQuery>,
    jar: CookieJar,
) -> impl IntoResponse {
    let player_id_str = jar
        .get("id")
        .map(|cookie| cookie.value().to_string())
        .expect("Cookie tidak ditemukan");

    let player_id: u32 = player_id_str.parse().unwrap();
    let room_id: u64 = params.room_id.parse().unwrap();

    {
        let mut instance = app.lock().await;
        match instance.put_player_to_room(player_id, room_id) {
            Ok(_) => {
                println!("{:#?}", instance.room_manager.room_players);
            }
            Err(err) => {
                return (StatusCode::BAD_REQUEST, err);
            }
        };
    }

    (StatusCode::OK, String::from("Berhasil masuk ke room"))
}
