use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone)]
pub enum CourtType {
    Jack,
    Queen,
    King,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum JokerType {
    Red,
    Black,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum CardIcon {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone)]
pub enum CardType {
    Ace,
    Court(CourtType),
    Spot(SpotNumber),
    Joker(JokerType),
}

#[derive(Debug, EnumIter, Clone)]
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

#[derive(Debug, Clone)]
pub struct Card {
    pub card_icon: Option<CardIcon>,
    pub card_type: CardType,
}

impl Card {
    pub fn generate_deck(with_joker: bool) -> Vec<Self> {
        let mut deck: Vec<Card> = Vec::new();

        for i in CardIcon::iter() {
            deck.push(Card {
                card_icon: Some(i.clone()),
                card_type: CardType::Ace,
            });

            for court_type in CourtType::iter() {
                deck.push(Card {
                    card_icon: Some(i.clone()),
                    card_type: CardType::Court(court_type),
                });
            }

            for spot_number in SpotNumber::iter() {
                deck.push(Card {
                    card_icon: Some(i.clone()),
                    card_type: CardType::Spot(spot_number),
                });
            }
        }

        if with_joker {
            for joker_type in JokerType::iter() {
                deck.push(Card {
                    card_icon: None,
                    card_type: CardType::Joker(joker_type.clone()),
                });
            }
        }

        return deck;
    }
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
