use axum::{
    Json, Router,
    extract::{Query, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;

use crate::{AppInstance, game::room_config::RoomConfig};

#[derive(Deserialize)]
pub struct UserPayload {
    id: String,
}

#[derive(Deserialize)]
pub struct RoomIdQuery {
    room_id: String,
}

pub async fn init(game_app: AppInstance) {
    let game_app1 = game_app.clone();
    let game_app2 = game_app.clone();
    let game_app3 = game_app.clone();

    let app: Router = Router::new()
        .route("/login", post(handle_login))
        .route("/ws", get(|jar, ws| handle_connect(game_app3, jar, ws)))
        .route(
            "/create-room",
            post(move |cookie_jar, config| handle_create_room(game_app1, cookie_jar, config)),
        )
        .route(
            "/join-room",
            post(move |cookie_jar, query_params| {
                handle_join_room(game_app2, query_params, cookie_jar)
            }),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6767")
        .await
        .unwrap();

    println!("Server berjalan di port 6767");

    if let Err(err) = axum::serve(listener, app).await {
        println!("{:?}", err);
    };
}

pub async fn handle_create_room(
    app: AppInstance,
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
        let mut instance = app.lock().unwrap();
        result = match instance.create_room(player_id, config) {
            Ok(res) => res,
            Err(err) => {
                return (StatusCode::BAD_REQUEST, err);
            }
        };
    }

    (
        StatusCode::OK,
        format!("Room berhasil dibuat, id: {}", result),
    )
}

pub async fn handle_join_room(
    app: AppInstance,
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
        let mut instance = app.lock().unwrap();
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

pub async fn handle_login(jar: CookieJar, Json(body): Json<UserPayload>) -> impl IntoResponse {
    let cookie = Cookie::build(("id", body.id))
        .path("/")
        .http_only(true)
        .build();

    let jar = jar.add(cookie);

    (jar, "Cookie telah dibuat")
}

pub async fn handle_connect(app: AppInstance, jar: CookieJar, ws: WebSocketUpgrade) -> Response {
    let name = jar
        .get("id")
        .map(|cookie| cookie.value().to_string())
        .expect("Cookie tidak ditemukan");

    ws.on_upgrade(|mut socket: axum::extract::ws::WebSocket| async move {
        println!("Pemain dengan ID {} terkoneksi", name);
        while let Some(msg) = socket.recv().await {
            let message = msg.unwrap();
            if (socket.send(message)).await.is_err() {
                return;
            }
        }

        println!("Client terputus");
    })
}
