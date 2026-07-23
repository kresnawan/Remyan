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
    let player_id: u32;
    let player_id_str = jar.get("id").map(|cookie| cookie.value().to_string());

    if let Some(value) = player_id_str {
        let pid = value.parse::<u32>();
        if let Ok(res) = pid {
            player_id = res;
        } else {
            return (StatusCode::BAD_REQUEST, format!("ID tidak valid"));
        }
    } else {
        return (StatusCode::BAD_REQUEST, format!("Cookie tidak ditemukan"));
    }

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

        server_instance
            .rooms
            .insert(result, ServerRoom::new(result));
    }

    (
        StatusCode::OK,
        format!("{}", str::from_utf8(&result).unwrap()),
    )
}

pub async fn handle_join_room(
    Extension(app): Extension<AppInstance>,
    Query(params): Query<RoomIdQuery>,
    jar: CookieJar,
) -> impl IntoResponse {
    let player_id: u32;
    let player_id_str = jar.get("id").map(|cookie| cookie.value().to_string());

    if let Some(value) = player_id_str {
        let pid = value.parse::<u32>();
        if let Ok(res) = pid {
            player_id = res;
        } else {
            return (StatusCode::BAD_REQUEST, format!("ID tidak valid"));
        }
    } else {
        return (StatusCode::BAD_REQUEST, format!("Cookie tidak ditemukan"));
    }

    let chars = params.room_id.as_bytes();

    if chars.len() < 6 {
        return (StatusCode::BAD_REQUEST, format!("Panjang id room terlalu pendek"));
    }

    let mut room_id = [0u8; 6];

    room_id.copy_from_slice(&chars[0..6]);

    let mut instance = app.lock().await;
    if let Err(err) = instance.put_player_to_room(player_id, room_id) {
        return (StatusCode::BAD_REQUEST, err);
    }

    (StatusCode::OK, String::from("Berhasil masuk ke room"))
}
