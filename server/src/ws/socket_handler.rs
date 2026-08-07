use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use remyan_core::protocol::event::{EventToken, RoomEvent, ServerEventPlayer};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::{
    AppInstance, ServerInstance,
    ws::command_handler::{handle_game_command, handle_room_command},
};

pub async fn handle_socket(
    socket: WebSocket,
    app: AppInstance,
    server: ServerInstance,
    player_id: u32,
    room_id: [u8; 6],
    name_alias: String,
) {
    println!(
        "Pemain dengan id {player_id} telah terkoneksi ke room: {}",
        str::from_utf8(&room_id).unwrap()
    );

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    {
        let mut server_instance = server.lock().await;
        let mut app_instance = app.lock().await;

        let app_room = app_instance.room_manager.rooms.get_mut(&room_id).unwrap();
        let room = server_instance.rooms.get_mut(&room_id).unwrap();

        let player = app_room.players.get_mut(&player_id).unwrap();
        player.name_alias = name_alias;

        let mapped_players: Vec<ServerEventPlayer> = app_room
            .players
            .iter()
            .clone()
            .map(|(id, player)| ServerEventPlayer {
                id: *id,
                name_alias: player.name_alias.clone(),
                score: player.current_score
            })
            .collect();

        room.txs.insert(player_id, Some(tx));
        room.broadcast(
            true,
            player_id,
            EventToken::RoomEvent(RoomEvent::RoomPlayer {
                players: mapped_players,
                host_id: app_room.host_id,
            }),
        )
        .await;
        room.send_player(
            EventToken::RoomEvent(RoomEvent::RoomConfig(app_room.config.clone())),
            player_id,
        )
        .await;
    }

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let cloned_app = app.clone();
    let cloned_server = server.clone();

    let mut receive_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Text(txt) => {
                    let currently_playing: bool;
                    let clone_app = Arc::clone(&cloned_app);
                    let clone_server = Arc::clone(&cloned_server);

                    {
                        let instance = clone_app.lock().await;
                        let room = instance.get_room(room_id).unwrap();
                        currently_playing = room.currently_playing;
                    }

                    if currently_playing {
                        handle_game_command(txt, player_id, room_id, clone_app, clone_server).await;
                    } else {
                        if false
                            == handle_room_command(txt, player_id, room_id, clone_app, clone_server)
                                .await
                        {
                            break;
                        }
                    }
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

    let mut app_instance = app.lock().await;
    let mut server_instance = server.lock().await;

    match app_instance.room_manager.remove_player_from_room(player_id) {
        Ok(len) => {
            if len == 0 {
                server_instance.rooms.remove(&room_id).unwrap();
            } else {
                let server_room = server_instance.rooms.get_mut(&room_id).unwrap();
                server_room.txs.remove(&player_id).unwrap();

                let app_room = app_instance.room_manager.rooms.get(&room_id).unwrap();

                let mapped_players: Vec<ServerEventPlayer> = app_room
                    .players
                    .iter()
                    .clone()
                    .map(|(id, player)| ServerEventPlayer {
                        id: *id,
                        name_alias: player.name_alias.clone(),
                        score: player.current_score
                    })
                    .collect();

                server_room
                    .broadcast(
                        true,
                        1,
                        EventToken::RoomEvent(RoomEvent::RoomPlayer {
                            players: mapped_players,
                            host_id: app_room.host_id,
                        }),
                    )
                    .await;
            }
        }

        Err(err) => {
            println!("{}", err);
        }
    }

    println!("Pemain {} telah meninggalkan room", player_id);
}
