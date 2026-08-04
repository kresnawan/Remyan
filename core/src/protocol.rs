use serde::{Deserialize, Serialize};

pub mod command;
pub mod event;

#[derive(Deserialize, Serialize, Debug,PartialEq, Clone)]
pub enum DrawSource {
    StockPile,
    DiscardPile,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Error {
    NotAHost,
    PlayerNotEnough,
    RoomIsCurrentlyPlaying,
    TooManyDraw,
    Ineligible,
    CardNotFound,
    RepeatTurn,
    InvalidCommand,
    RequireMeld,
    NotATurn,
    DiscardAJoker,
    AlreadyJoined,
    RoomNotFound,
    TooManyPlayers,
    DrawnCardRequired
}