use axum::extract::ws::Utf8Bytes;
use tokio::sync::MutexGuard;

use crate::{
    AppInstance,
    core::{
        app::App,
        protocol::{
            CommandToken, CommandTurn, DrawSource, Error, EventToken, EventTurn, GameCommand,
            GameEvent, RoomCommand, RoomEvent, ServerEvent,
        },
    },
    network::ws::{
        command_parser::parse_command,
        event_broadcaster::{broadcast, broadcast_card, ws_send_player},
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
                        broadcast(
                            room,
                            EventToken::RoomEvent(RoomEvent::StartGame),
                            player_id,
                            true,
                        )
                        .await;
                        broadcast(
                            room,
                            EventToken::GameEvent(GameEvent::CurrentTurn(
                                room.player_turns[room.current_turn.index],
                            )),
                            player_id,
                            true,
                        )
                        .await;
                        broadcast_card(room).await;
                        println!("{:#?}", room.discard_pile);
                        println!("{:#?}", room.stock_pile);
                        return Ok(());
                    }
                    Err(err) => {
                        ws_send_player(
                            room,
                            EventToken::ServerEvent(ServerEvent::Error(err)),
                            player_id,
                        )
                        .await;
                        return Err(());
                    }
                }
            }
            RoomCommand::EditConfig { new_config } => {}
            RoomCommand::SendMessage { message } => {
                let instance: MutexGuard<App> = app.lock().await;
                let room = instance.get_room(room_id).unwrap();

                broadcast(
                    room,
                    EventToken::RoomEvent(RoomEvent::Message {
                        message: message,
                        sender_id: player_id,
                    }),
                    player_id,
                    false,
                )
                .await;
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
) -> Result<GameEvent, Error> {
    let command = parse_command(command);
    let result: Result<GameEvent, Error>;

    if let Ok(CommandToken::GameCommand(token)) = command {
        let mut instance = app.lock().await;
        let room = instance.room_manager.rooms.get_mut(&room_id).unwrap();

        match token {
            GameCommand::Turn(turn) => {
                if room.player_turns[room.current_turn.index] != player_id {
                    return Err(Error::NotATurn);
                }

                match turn {
                    CommandTurn::Discard(card) => {
                        if let Some(_) = room.current_turn.discarded_card {
                            return Err(Error::RepeatTurn);
                        }
                        match room.handle_discard(player_id, card) {
                            Ok(res) => {
                                return Ok(GameEvent::Turn(EventTurn::Discard {
                                    player_id,
                                    card: res,
                                }));
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    }
                    CommandTurn::Draw(draw) => {
                        if let Some(_) = room.current_turn.drawn_card {
                            return Err(Error::RepeatTurn);
                        }

                        let res = match draw {
                            DrawSource::DiscardPile(number) => {
                                match room
                                    .handle_draw_from_discard_pile(usize::from(number), player_id)
                                    .await
                                {
                                    Ok(res) => {
                                        return Ok(GameEvent::Turn(EventTurn::Draw {
                                            player_id,
                                            source: DrawSource::DiscardPile(number),
                                        }));
                                    }
                                    Err(err) => {
                                        return Err(err);
                                    }
                                }
                            }
                            DrawSource::StockPile => {
                                match room.handle_draw_from_stock_pile(player_id).await {
                                    Ok(res) => {
                                        return Ok(GameEvent::Turn(EventTurn::Draw {
                                            player_id,
                                            source: DrawSource::StockPile,
                                        }));
                                    }
                                    Err(err) => {
                                        return Err(err);
                                    }
                                }
                            }
                        };
                    }
                }
            }
            GameCommand::Make { cards } => match room.handle_meld(player_id, cards) {
                Ok(res) => {
                    return Ok(GameEvent::Make {
                        player_id,
                        cards: res,
                    });
                }
                Err(err) => {
                    return Err(err);
                }
            },
            GameCommand::Put { cards } => match room.handle_put(player_id, cards) {
                Ok(res) => {
                    return Ok(GameEvent::Put {
                        player_id,
                        cards: res,
                    });
                }
                Err(err) => {
                    return Err(err);
                }
            },
        };

        // if true == room.try_next_turn() {
        //     room.current_turn.reset();
        //     broadcast(
        //         room,
        //         EventToken::GameEvent(GameEvent::CurrentTurn(
        //             room.player_turns[room.current_turn.index],
        //         )),
        //         player_id,
        //         true,
        //     )
        //     .await
        //     .unwrap();
        // };

        // return result;
    }

    return Err(Error::InvalidCommand);
}
