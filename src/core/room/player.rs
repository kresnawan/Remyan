use crate::{Tx, core::card::Card};



#[derive(Debug)]
pub struct RoomPlayer {
    pub current_score: i32,
    pub hand_cards: Vec<Card>,
    pub melded_cards: Vec<Vec<Card>>,
    pub putted_cards: Vec<Card>,
    pub tx: Option<Tx>,
}

impl RoomPlayer {
    pub fn new() -> Self {
        RoomPlayer {
            current_score: 0,
            hand_cards: Vec::new(),
            melded_cards: Vec::new(),
            putted_cards: Vec::new(),
            tx: None,
        }
    }
}