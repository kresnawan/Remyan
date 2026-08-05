use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
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

impl CardIcon {
    pub fn from_number(number: usize) -> Option<CardIcon> {
        match number {
            0 => Some(CardIcon::Spade),
            1 => Some(CardIcon::Heart),
            2 => Some(CardIcon::Diamond),
            3 => Some(CardIcon::Club),

            _ => None
        }
    }
    pub fn as_number(&self) -> usize {
        match self {
            CardIcon::Spade => 0,
            CardIcon::Heart => 1,
            CardIcon::Diamond => 2,
            CardIcon::Club => 3
        }
    } 
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
    pub fn new(card_icon: Option<CardIcon>, card_type: CardType) -> Card {
        Card {
            card_icon,
            card_type,
        }
    }
    pub fn get_unmelded_weight(&self) -> u32 {
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

    pub fn get_meld_weight(&self) -> u32 {
        match &self.card_type {
            CardType::Joker(_) => {
                return 5;
            }
            CardType::Ace => {
                return self.get_unmelded_weight();
            }
            CardType::Court(_) => {
                return self.get_unmelded_weight();
            }
            CardType::Spot(_) => {
                return self.get_unmelded_weight();
            }
        }
    }

    pub fn get_hit_weight(&self) -> u32 {
        self.get_unmelded_weight() * 10
    }

    pub fn get_spot_index(&self) -> Option<usize> {
        for (index, sn) in SpotNumber::iter().enumerate() {
            if let CardType::Spot(n) = self.card_type {
                if n == sn {
                    return Some(index);
                }
            }
        }

        return None;
    }

    pub fn get_court_index(&self) -> Option<usize> {
        for (index, sn) in CourtType::iter().enumerate() {
            if let CardType::Court(n) = self.card_type {
                if n == sn {
                    return Some(index);
                }
            }
        }

        return None;
    }

    pub fn is_court(&self) -> bool {
        if let CardType::Court(_) = self.card_type {
            return true;
        }

        false
    }

    pub fn is_joker(&self) -> bool {
        if let CardType::Joker(_) = &self.card_type {
            return true
        }

        return false
    }

    pub fn get_color_type(&self) -> Option<JokerType> {
        if let Some(icon) = self.card_icon {
            if icon == CardIcon::Club || icon == CardIcon::Spade {
                return Some(JokerType::Black);
            } else {
                return Some(JokerType::Red);
            }
        }

        return None;
    }

    pub fn joker(joker_type: JokerType) -> Card {
        Card { card_icon: None, card_type: CardType::Joker(joker_type) }
    }
}
