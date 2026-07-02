use axum::extract::ws::Utf8Bytes;
use remyan_core::{
    AppInstance,
    protocol::{
        DrawSource, Error,
        command::{CommandToken, GameCommand, RoomCommand, TurnCommand},
        event::{EventToken, GameEvent, RoomEvent, ServerEvent, TurnEvent},
    },
};

use crate::{ServerInstance, ws::command_parser::parse_command};

pub async fn handle_room_command(
    command: Utf8Bytes,
    player_id: u32,
    room_id: u64,
    app: AppInstance,
    server: ServerInstance,
) {
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
                            player_id,
                            EventToken::GameEvent(GameEvent::CurrentTurn(
                                room.player_turns[room.current_turn.index],
                            )),
                        )
                        .await;
                    server_room.broadcast_card(&room).await;
                }
                Err(err) => {
                    server_room
                        .send_player(EventToken::ServerEvent(ServerEvent::Error(err)), player_id)
                        .await;
                    return;
                }
            },
            RoomCommand::EditConfig { new_config } => {}
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
        }
    } else {
        server_room
            .send_player(
                EventToken::ServerEvent(ServerEvent::Error(Error::InvalidCommand)),
                player_id,
            )
            .await;
        return;
    }
}

pub async fn handle_game_command(
    command: Utf8Bytes,
    player_id: u32,
    room_id: u64,
    app: AppInstance,
    server: ServerInstance,
) {
    let mut server_instance = server.lock().await;
    let server_room = server_instance.rooms.get_mut(&room_id).unwrap();

    let command = parse_command(command);

    if let Ok(CommandToken::GameCommand(token)) = command {
        let mut instance = app.lock().await;
        let room = instance.room_manager.rooms.get_mut(&room_id).unwrap();

        match token {
            GameCommand::Turn(turn) => {
                if room.player_turns[room.current_turn.index] != player_id {
                    server_room
                        .send_player(
                            EventToken::ServerEvent(ServerEvent::Error(Error::NotATurn)),
                            player_id,
                        )
                        .await;
                    return;
                }

                match turn {
                    TurnCommand::Discard(card) => {
                        if let Some(_) = room.current_turn.discarded_card {
                            server_room
                                .send_player(
                                    EventToken::ServerEvent(ServerEvent::Error(Error::RepeatTurn)),
                                    player_id,
                                )
                                .await;
                            return;
                        }
                        match room.handle_discard(player_id, card) {
                            Ok(res) => {
                                server_room
                                    .broadcast(
                                        false,
                                        player_id,
                                        EventToken::GameEvent(GameEvent::Turn(
                                            TurnEvent::Discard {
                                                player_id,
                                                card: res,
                                            },
                                        )),
                                    )
                                    .await;
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
                    TurnCommand::Draw(draw) => {
                        match draw {
                            DrawSource::DiscardPile => {
                                match room
                                    .handle_draw_from_discard_pile(player_id)
                                    .await
                                {
                                    Ok(res) => {
                                        server_room
                                            .broadcast(
                                                false,
                                                player_id,
                                                EventToken::GameEvent(GameEvent::Turn(
                                                    TurnEvent::Draw {
                                                        player_id,
                                                        source: DrawSource::DiscardPile,
                                                    },
                                                )),
                                            )
                                            .await;
                                        server_room
                                            .send_player(
                                                EventToken::ServerEvent(ServerEvent::DrawnCard(res)),
                                                player_id,
                                            )
                                            .await;
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
                            DrawSource::StockPile => {
                                match room.handle_draw_from_stock_pile(player_id).await {
                                    Ok(res) => {
                                        server_room
                                            .broadcast(
                                                false,
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
                                                EventToken::ServerEvent(ServerEvent::DrawnCard(res)),
                                                player_id,
                                            )
                                            .await;
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
                        };
                    }
                }
            }
            GameCommand::Make { cards } => match room.handle_meld(player_id, cards) {
                Ok(res) => {
                    server_room
                        .broadcast(
                            false,
                            player_id,
                            EventToken::GameEvent(GameEvent::Make {
                                player_id,
                                cards: res,
                            }),
                        )
                        .await;
                }
                Err(err) => {
                    server_room
                        .send_player(EventToken::ServerEvent(ServerEvent::Error(err)), player_id)
                        .await;
                    return;
                }
            },
            GameCommand::Put { cards } => match room.handle_put(player_id, cards) {
                Ok(res) => {
                    server_room
                        .broadcast(
                            false,
                            player_id,
                            EventToken::GameEvent(GameEvent::Put {
                                player_id,
                                cards: res,
                            }),
                        )
                        .await;
                }
                Err(err) => {
                    server_room
                        .send_player(EventToken::ServerEvent(ServerEvent::Error(err)), player_id)
                        .await;
                    return;
                }
            },
        };

        println!("Pengecekan turn dijalankan");
        if true == room.try_next_turn() {
            room.current_turn.reset();
            server_room
                .broadcast(
                    true,
                    player_id,
                    EventToken::GameEvent(GameEvent::CurrentTurn(
                        room.player_turns[room.current_turn.index],
                    )),
                )
                .await;
        };
    } else {
        server_room
            .send_player(
                EventToken::ServerEvent(ServerEvent::Error(Error::InvalidCommand)),
                player_id,
            )
            .await;
        return;
    }
}
