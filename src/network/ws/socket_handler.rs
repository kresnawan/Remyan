use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use tokio::sync::mpsc;

use crate::{
    AppInstance,
    network::ws::command_handler::{handle_game_command, handle_room_command},
};
use futures_util::{SinkExt, StreamExt};

pub async fn handle_socket(socket: WebSocket, app: AppInstance, player_id: u32, room_id: u64) {
    println!("Pemain dengan id {player_id} telah terkoneksi ke room: {room_id}");

    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    {
        let mut instance = app.lock().await;
        let chosen_room = instance.room_manager.rooms.get_mut(&room_id).unwrap();

        let player = chosen_room.players.get_mut(&player_id).unwrap();
        player.tx = Some(tx);
    }

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let mut receive_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Text(txt) => {
                    let currently_playing: bool;
                    let clone_app = Arc::clone(&app);

                    {
                        let instance = clone_app.lock().await;
                        let room = instance.get_room(room_id).unwrap();
                        currently_playing = room.currently_playing;
                    }

                    if currently_playing {
                        handle_game_command(txt, player_id, room_id, clone_app).await;
                    } else {
                        handle_room_command(txt, player_id, room_id, clone_app).await;
                    }

                    // match result {
                    //     Ok(res) => {
                    //         broadcast(room, EventToken::GameEvent(res), player_id, false).await;
                    //     }
                    //     Err(err) => {
                    //         ws_send_player(room, EventToken::ServerEvent(Error(err)), player_id)
                    //             .await;
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
}
