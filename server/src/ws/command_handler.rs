use std::time::Duration;

use axum::extract::ws::Utf8Bytes;
use remyan_core::protocol::{
    DrawSource, Error,
    command::{CommandToken, GameCommand, RoomCommand, TurnCommand},
    event::{EventToken, GameEvent, RoomEvent, ServerEvent, TurnEvent},
};

use crate::{AppInstance, ServerInstance, ws::command_parser::parse_command};

pub async fn handle_room_command(
    command: Utf8Bytes,
    player_id: u32,
    room_id: [u8; 6],
    app: AppInstance,
    server: ServerInstance,
) -> bool {
    let mut instance = app.lock().await;
    let room = instance.room_manager.rooms.get_mut(&room_id).unwrap();

    let mut server_instance = server.lock().await;
    let server_room = server_instance.rooms.get_mut(&room_id).unwrap();

    let command = parse_command(command);

    if let Ok(CommandToken::RoomCommand(token)) = command {
        match token {
            RoomCommand::StartGame => match room.start_game(1, player_id) {
                Ok(_) => {
                    server_room
                        .broadcast(true, player_id, EventToken::RoomEvent(RoomEvent::StartGame))
                        .await;
                    server_room
                        .broadcast(
                            true,
                            0,
                            EventToken::GameEvent(GameEvent::PlayersTurn(
                                room.player_turns.clone(),
                            )),
                        )
                        .await;
                    server_room.broadcast_card(&room).await;
                    server_room
                        .broadcast(
                            true,
                            player_id,
                            EventToken::GameEvent(GameEvent::CurrentTurn(
                                room.player_turns[room.current_turn.index],
                            )),
                        )
                        .await;
                }
                Err(err) => {
                    server_room
                        .send_player(EventToken::ServerEvent(ServerEvent::Error(err)), player_id)
                        .await;
                }
            },
            RoomCommand::EditConfig { new_config } => {
                // Only host could change the room config
                if let Err(e) = room.edit_config(new_config.clone(), player_id) {
                    server_room
                        .send_player(EventToken::ServerEvent(ServerEvent::Error(e)), player_id)
                        .await;
                    return true;
                }

                server_room
                    .broadcast(
                        true,
                        player_id,
                        EventToken::RoomEvent(RoomEvent::RoomConfig(new_config)),
                    )
                    .await;
            }
            RoomCommand::SendMessage { message } => {
                server_room
                    .broadcast(
                        false,
                        player_id,
                        EventToken::RoomEvent(RoomEvent::Message {
                            message: message,
                            sender_id: player_id,
                        }),
                    )
                    .await;
            }
            RoomCommand::LeaveRoom => {
                return false;
            }
        }
    } else {
        server_room
            .send_player(
                EventToken::ServerEvent(ServerEvent::Error(Error::InvalidCommand)),
                player_id,
            )
            .await;
    }

    return true;
}

pub async fn handle_game_command(
    command: Utf8Bytes,
    player_id: u32,
    room_id: [u8; 6],
    app: AppInstance,
    server: ServerInstance,
) {
    let command = parse_command(command);

    if let Ok(CommandToken::GameCommand(token)) = command {
        let current_counter = {
            let mut server_instance = server.lock().await;
            let server_room = server_instance.rooms.get_mut(&room_id).unwrap();
            let mut instance = app.lock().await;
            let room = instance.room_manager.rooms.get_mut(&room_id).unwrap();

            match token {
                GameCommand::Turn(turn) => match turn {
                    TurnCommand::Discard(card) => match room.handle_discard(player_id, card) {
                        Ok(res) => {
                            server_room
                                .broadcast(
                                    true,
                                    player_id,
                                    EventToken::GameEvent(GameEvent::Turn(TurnEvent::Discard {
                                        player_id,
                                        card: res,
                                    })),
                                )
                                .await;
                            server_room.counter += 1;
                        }
                        Err(err) => {
                            server_room
                                .send_player(
                                    EventToken::ServerEvent(ServerEvent::Error(err)),
                                    player_id,
                                )
                                .await;
                            return;
                        }
                    },
                    TurnCommand::Draw(draw) => {
                        if let DrawSource::DiscardPile = draw {
                            match room.handle_draw_from_discard_pile(player_id) {
                                Ok(_) => {
                                    server_room
                                        .broadcast(
                                            true,
                                            player_id,
                                            EventToken::GameEvent(GameEvent::Turn(
                                                TurnEvent::Draw {
                                                    player_id,
                                                    source: DrawSource::DiscardPile,
                                                },
                                            )),
                                        )
                                        .await;
                                    server_room.counter += 1;
                                }
                                Err(err) => {
                                    server_room
                                        .send_player(
                                            EventToken::ServerEvent(ServerEvent::Error(err)),
                                            player_id,
                                        )
                                        .await;
                                    return;
                                }
                            }
                        }
                        if let DrawSource::StockPile = draw {
                            match room.handle_draw_from_stock_pile(player_id) {
                                Ok(res) => {
                                    server_room
                                        .broadcast(
                                            true,
                                            player_id,
                                            EventToken::GameEvent(GameEvent::Turn(
                                                TurnEvent::Draw {
                                                    player_id,
                                                    source: DrawSource::StockPile,
                                                },
                                            )),
                                        )
                                        .await;
                                    server_room
                                        .send_player(
                                            EventToken::GameEvent(GameEvent::DrawnCard(res)),
                                            player_id,
                                        )
                                        .await;
                                    server_room.counter += 1;
                                }
                                Err(err) => {
                                    server_room
                                        .send_player(
                                            EventToken::ServerEvent(ServerEvent::Error(err)),
                                            player_id,
                                        )
                                        .await;
                                    return;
                                }
                            }
                        }
                    }
                },
                GameCommand::Meld { cards } => match room.handle_meld(player_id, cards) {
                    Ok(res) => {
                        server_room
                            .broadcast(
                                true,
                                player_id,
                                EventToken::GameEvent(GameEvent::Meld {
                                    player_id,
                                    cards: res,
                                }),
                            )
                            .await;
                        if room.player_turns[room.current_turn.index] == player_id {
                            server_room.counter += 1;
                        } else {
                            return;
                        }
                    }
                    Err(err) => {
                        server_room
                            .send_player(
                                EventToken::ServerEvent(ServerEvent::Error(err)),
                                player_id,
                            )
                            .await;
                        return;
                    }
                },
            }

            server_room.counter
        };

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;

            let mut server_instance = server.lock().await;
            let mut app_instance = app.lock().await;

            let server_room = server_instance.rooms.get_mut(&room_id).unwrap();
            let app_room = app_instance.room_manager.rooms.get_mut(&room_id).unwrap();

            if server_room.counter == current_counter {
                if let Some(turn_done) = app_room.try_next_turn() {
                    if turn_done {
                        app_room.current_turn.reset();
                        server_room
                            .broadcast(
                                true,
                                player_id,
                                EventToken::GameEvent(GameEvent::CurrentTurn(
                                    app_room.player_turns[app_room.current_turn.index],
                                )),
                            )
                            .await;
                    }
                } else {
                    app_room.current_turn.reset();
                    server_room
                        .broadcast(true, player_id, EventToken::RoomEvent(RoomEvent::GameEnded))
                        .await;
                }
            }
        });
    } else {
        let server_instance = server.lock().await;
        let server_room = server_instance.rooms.get(&room_id).unwrap();
        server_room
            .send_player(
                EventToken::ServerEvent(ServerEvent::Error(Error::InvalidCommand)),
                player_id,
            )
            .await;
        return;
    }
}
