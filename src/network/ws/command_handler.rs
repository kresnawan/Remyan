use axum::extract::ws::Utf8Bytes;

use crate::{
    AppInstance,
    network::ws::{
        command_parser::parse_command,
        token::{
            command::{CommandToken, RoomCommand},
            event::{
                Error,
                EventToken::{self},
                GameEvent,
                RoomEvent::{self},
                ServerEvent::{self},
            },
        },
    },
};

pub async fn handle_room_command(
    command: Utf8Bytes,
    player_id: u32,
    room_id: u64,
    app: AppInstance,
) -> Result<(), ()> {
    let command = match parse_command(command) {
        Ok(res) => res,
        Err(_) => return Err(()),
    };

    if let CommandToken::RoomCommand(token) = command {
        match token {
            RoomCommand::StartGame => {
                let mut instance = app.lock().await;
                let room = instance.room_manager.rooms.get_mut(&room_id).unwrap();

                match room.start_game(1, player_id) {
                    Ok(_) => {
                        room.broadcast(
                            EventToken::RoomEvent(RoomEvent::StartGame),
                            player_id,
                            true,
                        )
                        .unwrap();
                        room.broadcast(
                            EventToken::GameEvent(GameEvent::CurrentTurn(
                                room.player_turns[room.current_turn.index],
                            )),
                            player_id,
                            true,
                        )
                        .unwrap();
                        room.broadcast_card().unwrap();
                        return Ok(());
                    }
                    Err(err) => {
                        room.ws_send_player(
                            EventToken::ServerEvent(ServerEvent::Error(err)),
                            player_id,
                        )
                        .unwrap();
                        return Err(());
                    }
                }
            }
            RoomCommand::EditConfig { new_config } => {}
            RoomCommand::SendMessage { message } => {
                let instance = app.lock().await;
                let room = instance.get_room(room_id).unwrap();

                room.broadcast(
                    EventToken::RoomEvent(RoomEvent::Message {
                        message: message,
                        sender_id: player_id,
                    }),
                    player_id,
                    false,
                )
                .unwrap();
            }
        }
    }

    Ok(())
}

pub async fn handle_game_command(
    command: Utf8Bytes,
    player_id: u32,
    room_id: u64,
    app: AppInstance,
) -> Result<(), Error> {
    let command = match parse_command(command) {
        Ok(res) => res,
        Err(_) => return Err(Error::InvalidCommand),
    };

    if let CommandToken::GameCommand(token) = command {
        let mut instance = app.lock().await;
        let room = instance.room_manager.rooms.get_mut(&room_id).unwrap();

        match room.handle_turn(token, player_id) {
            Ok(event) => {
                room.broadcast(EventToken::GameEvent(event), player_id, false)
                    .unwrap();
            }
            Err(err) => {
                room.ws_send_player(EventToken::ServerEvent(ServerEvent::Error(err)), player_id)
                    .unwrap();
            }
        };

        if true == room.try_next_turn() {
            room.current_turn.reset();
            room.broadcast(
                EventToken::GameEvent(GameEvent::CurrentTurn(
                    room.player_turns[room.current_turn.index],
                )),
                player_id,
                true,
            )
            .unwrap();
        };
    }

    Ok(())
}
