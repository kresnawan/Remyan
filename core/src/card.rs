use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum CourtType {
    Jack,
    Queen,
    King,
}

#[derive(Debug, EnumIter, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum JokerType {
    Red,
    Black,
}

#[derive(Debug, EnumIter, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum CardIcon {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum CardType {
    Ace,
    Court(CourtType),
    Spot(SpotNumber),
    Joker(JokerType),
}

#[derive(Debug, EnumIter, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum SpotNumber {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Card {
    pub card_icon: Option<CardIcon>,
    pub card_type: CardType,
}

impl Card {
    pub fn get_card_power(&self) -> u32 {
        match &self.card_type {
            CardType::Joker(_) => {
                return 25;
            }
            CardType::Ace => {
                return 15;
            }
            CardType::Court(_) => {
                return 10;
            }
            CardType::Spot(_) => {
                return 5;
            }
        }
    }
}
