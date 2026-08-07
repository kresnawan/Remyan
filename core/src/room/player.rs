use crate::{Card};

#[derive(Debug)]
pub struct RoomPlayer {
    pub current_score: i32,
    pub hand_cards: Vec<Card>,
    pub melded_cards: Vec<Vec<Card>>,
    pub name_alias: String
}

impl RoomPlayer {
    pub fn new() -> Self {
        RoomPlayer {
            current_score: 0,
            hand_cards: Vec::new(),
            melded_cards: Vec::new(),
            name_alias: String::new()
        }
    }
}