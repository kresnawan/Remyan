use serde::{Deserialize, Serialize};

use crate::core::{card::Card, room::RoomConfig};

#[derive(Deserialize, Debug)]
pub enum CommandToken {
    RoomCommand(RoomCommand),
    GameCommand(GameCommand)
}

#[derive(Deserialize, Debug)]
pub enum RoomCommand {
    StartGame,
    SendMessage { message: String },
    EditConfig { new_config: RoomConfig },
}

#[derive(Deserialize, Debug)]
pub enum GameCommand {
    Put {
        cards: Vec<Card>
    },
    Make {
        cards: Vec<Card>
    },
    Turn(CommandTurn)
}

#[derive(Deserialize, Serialize, Debug)]
pub enum DrawSource {
    StockPile,
    DiscardPile(u8),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum CommandTurn {
    Draw(DrawSource),
    Discard(Card),
}