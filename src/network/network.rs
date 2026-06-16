use std::{collections::HashMap, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use tokio::sync::{RwLock, mpsc};

use axum::{
    Json, Router,
    extract::{
        Query, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;

use crate::{
    AppInstance, Connections,
    game::{room::RoomPlayer, room_config::RoomConfig},
};

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

    let peers: Connections = Arc::new(RwLock::new(HashMap::new()));

    let app: Router = Router::new()
        .route("/login", post(handle_login))
        .route(
            "/ws",
            get(move |jar, query, ws| handle_connect(peers, game_app3, jar, query, ws)),
        )
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
        let mut instance = app.lock().await;
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

pub async fn handle_login(jar: CookieJar, Json(body): Json<UserPayload>) -> impl IntoResponse {
    let cookie = Cookie::build(("id", body.id))
        .path("/")
        .http_only(true)
        .build();

    let jar = jar.add(cookie);

    (jar, "Cookie telah dibuat")
}

pub async fn handle_connect(
    connections: Connections,
    game_app: AppInstance,
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

    ws.on_upgrade(move |socket| handle_socket(socket, connections, game_app, player_id, room_id))
}

async fn handle_socket(
    socket: WebSocket,
    state: Connections,
    app: AppInstance,
    player_id: u32,
    room_id: u64,
) {
    println!("Pemain dengan id {player_id} telah terkoneksi ke room: {room_id}");

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    // {
    //     state.write().await.insert(player_id, tx);
    // }

    {
        let mut instance = app.lock().await;
        let chosen_room = instance.room_manager.rooms.get_mut(&room_id).unwrap();

        chosen_room
            .players
            .insert(
                player_id,
                RoomPlayer {
                    current_score: 0,
                    card_stack: Vec::new(),
                    tx: Some(tx),
                },
            )
            .unwrap();
    }

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // let state_clone = state.clone();
    let mut receive_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Text(txt) => {
                    let msg: String = txt.to_string();

                    if msg.starts_with("bc/") {
                        let pesan: Vec<&str> = msg.split("/").collect();
                        let isi = pesan[1];

                        let instance = app.lock().await;
                        let room = instance.room_manager.rooms.get(&room_id).unwrap();

                        for (&pid, pd) in room.players.iter() {
                            if pid != player_id {
                                let ptx = match pd.tx.as_ref() {
                                    Some(tx) => tx,
                                    None => {
                                        continue;
                                    }
                                };

                                if let Err(_) = ptx.send(Message::Text(isi.into())) {
                                    continue;
                                }
                            }
                        }
                    }

                    // println!("Player {player_id}: {txt}");
                    // let players_tx = state_clone.read().await;

                    // for (&pid, ptx) in players_tx.iter() {
                    //     if pid != player_id {
                    //         ptx.send(Message::Text(format!("{pid}: {txt}").into()))
                    //             .expect("Gagal mengirim pesan kembali ke pengguna");
                    //     }
                    // }
                }

                Message::Close(_) => {
                    break;
                }

                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => receive_task.abort(),
        _ = (&mut receive_task) => send_task.abort(),
    }

    println!("Koneksi pemain terputus: {player_id}");
    state.write().await.remove(&player_id);
}
