use axum::extract::ws::Utf8Bytes;

use crate::{
    AppInstance,
    network::ws::{
        command_parser::parse_command,
        token::{
            command::{CommandToken, CommandTurn, DrawSource, GameCommand, RoomCommand},
            event::{
                EventToken::{self},
                EventTurn, GameEvent,
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

                match room.start_game(1) {
                    Ok(_) => {
                        room.broadcast(
                            EventToken::RoomEvent(RoomEvent::StartGame),
                            player_id,
                            true,
                        )
                        .unwrap();
                        room.broadcast(
                            EventToken::GameEvent(GameEvent::CurrentTurn(
                                room.player_turns[room.current_turn],
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
) -> Result<(), ()> {
    let command = match parse_command(command) {
        Ok(res) => res,
        Err(_) => return Err(()),
    };

    if let CommandToken::GameCommand(token) = command {
        match token {
            GameCommand::Turn(turn) => match turn {
                CommandTurn::Draw(src) => match src {
                    DrawSource::DiscardPile(count) => {
                        let mut instance = app.lock().await;
                        let room = instance.room_manager.rooms.get_mut(&room_id).unwrap();

                        if usize::from(count) > room.players.len() - 1 {
                            return Err(());
                        }

                        if room.discard_pile.len() < usize::from(count) {
                            return Err(());
                        }

                        let player = room.players.get_mut(&player_id).unwrap();

                        for _ in 0..count {
                            if let Some(n) = room.discard_pile.pop() {
                                player.hand_cards.push(n);
                            }
                        }

                        room.broadcast(
                            EventToken::GameEvent(GameEvent::Turn(EventTurn::Draw {
                                player_id,
                                source: DrawSource::DiscardPile(count),
                            })),
                            player_id,
                            false,
                        )
                        .unwrap();
                    }
                    DrawSource::StockPile => {
                        let mut instance = app.lock().await;
                        let room = instance.room_manager.rooms.get_mut(&room_id).unwrap();
                        let player = room.players.get_mut(&player_id).unwrap();

                        if let Some(card) = room.stock_pile.pop() {
                            player.hand_cards.push(card);
                        }

                        room.broadcast(
                            EventToken::GameEvent(GameEvent::Turn(EventTurn::Draw {
                                player_id,
                                source: DrawSource::StockPile,
                            })),
                            player_id,
                            false,
                        )
                        .unwrap();
                    }
                },
                CommandTurn::Discard(card) => {
                    let mut instance = app.lock().await;
                    let room = instance.room_manager.rooms.get_mut(&room_id).unwrap();
                    let player = room.players.get_mut(&player_id).unwrap();

                    let card_index = player.hand_cards.iter().position(|ca| ca == &card);
                    if let Some(index) = card_index {
                        let discarded_card = player.hand_cards.remove(index);
                        room.discard_pile.push(discarded_card);

                        room.broadcast(
                            EventToken::GameEvent(GameEvent::Turn(EventTurn::Discard {
                                player_id: player_id,
                                card: discarded_card,
                            })),
                            player_id,
                            false,
                        )
                        .unwrap();
                    }
                }
            },
            GameCommand::Make { cards } => {}
            GameCommand::Put { cards } => {}
        }
    }

    Ok(())
}
