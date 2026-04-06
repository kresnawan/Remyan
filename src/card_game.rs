use crate::Card;

#[derive(Debug)]
pub struct CardGame {
    pub card_stack: Vec<Card>,
    pub bookie_id: u32,
}
