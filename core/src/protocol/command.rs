use serde::{Deserialize, Serialize};

use crate::{card::Card, protocol::DrawSource, room::RoomConfig};

#[derive(Deserialize, Serialize, Debug)]
pub enum CommandToken {
    RoomCommand(RoomCommand),
    GameCommand(GameCommand)
}

#[derive(Deserialize, Serialize, Debug)]
pub enum RoomCommand {
    StartGame,
    SendMessage { message: String },
    EditConfig { new_config: RoomConfig },
    LeaveRoom
}

#[derive(Deserialize, Serialize, Debug)]
pub enum GameCommand {
    Put {
        cards: Vec<Card>
    },
    Make {
        cards: Vec<Card>
    },
    Turn(TurnCommand)
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TurnCommand {
    Draw(DrawSource),
    Discard(Card),
}