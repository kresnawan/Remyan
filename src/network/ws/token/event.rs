use crate::{game::card::Card, network::ws::token::command::DrawSource};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum EventToken {
    RoomEvent(RoomEvent),
    GameEvent(GameEvent),
    ServerEvent(ServerEvent),
}

#[derive(Serialize, Debug)]
pub enum ServerEvent {
    Error(Error),
    PlayerCard(Vec<Card>),
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

#[derive(Serialize, Debug)]
pub enum RoomEvent {
    StartGame,
    Message { message: String, sender_id: u32 },
    EditConfig,
}

#[derive(Serialize, Debug)]
pub enum GameEvent {
    Put { player_id: u32, cards: Vec<Card> },
    Make { player_id: u32, cards: Vec<Card> },
    Turn(EventTurn),
    CurrentTurn(u32),
    DrawnCard { cards: Vec<Card> },
}

#[derive(Serialize, Debug)]
pub enum EventTurn {
    Draw { player_id: u32, source: DrawSource },
    Discard { player_id: u32, card: Card },
}
