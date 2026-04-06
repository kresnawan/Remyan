use crate::{Card, CardIcon, CardType, CourtType, JokerType, SpotNumber};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(with_joker: bool) -> Self {
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

        return Self { cards: deck };
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }
}
