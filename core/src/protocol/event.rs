use serde::{Deserialize, Serialize};

use crate::{Card, RoomConfig, protocol::{DrawSource, Error}};

#[derive(Deserialize, Serialize, Debug)]
pub enum EventToken {
    RoomEvent(RoomEvent),
    GameEvent(GameEvent),
    ServerEvent(ServerEvent),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerEventPlayer {
    pub id: u32,
    pub name_alias: String
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ServerEvent {
    Error(Error),
    PlayerCard(Vec<Card>),
    DrawnCard(Card),
    
}

#[derive(Deserialize, Serialize, Debug)]
pub enum RoomEvent {
    StartGame,
    Message { message: String, sender_id: u32 },
    RoomPlayer{ players: Vec<u32>, host_id: u32 },
    RoomConfig(RoomConfig),
    GameEnded
}

#[derive(Deserialize, Serialize, Debug)]
pub enum GameEvent {
    Put { player_id: u32, cards: Vec<Card> },
    Make { player_id: u32, cards: Vec<Card> },
    Turn(TurnEvent),
    CurrentTurn(u32),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TurnEvent {
    Draw { player_id: u32, source: DrawSource },
    Discard { player_id: u32, card: Card },
}
