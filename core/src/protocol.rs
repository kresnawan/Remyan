use serde::{Deserialize, Serialize};

pub mod command;
pub mod event;

#[derive(Deserialize, Serialize, Debug,PartialEq)]
pub enum DrawSource {
    StockPile,
    DiscardPile,
}

#[derive(Serialize, Debug)]
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
}