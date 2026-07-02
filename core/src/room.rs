use rand::RngExt;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

mod config;
mod manager;
mod player;

pub use config::*;
pub use manager::*;
pub use player::*;

use crate::{
    Card, CardGame, CardType, CourtType, Deck, PlayerTurn, SpotNumber,
    protocol::{DrawSource, Error},
};

#[derive(Debug)]
pub struct Room {
    pub deck: Deck,
    pub stock_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub room_id: u64,
    pub games: HashMap<u32, CardGame>,
    pub players: HashMap<u32, RoomPlayer>,
    pub player_turns: Vec<u32>,
    pub config: RoomConfig,
    pub host_id: u32,
    pub currently_playing: bool,
    pub current_turn: PlayerTurn,
}

impl Room {
    pub fn new(cfg: RoomConfig, host_id: u32) -> Result<Self, String> {
        let new_session_player = RoomPlayer::new();
        let deck = Deck::new(cfg.with_joker);

        let random_number: u64 = rand::rng().random();
        let mut players = HashMap::new();
        players.insert(host_id, new_session_player);

        return Ok(Self {
            deck: deck,
            stock_pile: Vec::new(),
            discard_pile: Vec::new(),
            room_id: random_number,
            games: HashMap::new(),
            players: players,
            player_turns: Vec::new(),
            config: cfg,
            host_id: host_id,
            currently_playing: false,
            current_turn: PlayerTurn::new(),
        });
    }

    fn share_cards(&mut self) {
        self.deck.shuffle();
        // Share cards
        for (_, obj) in self.players.iter_mut() {
            for _ in 0..6 {
                let card = self.deck.cards.pop().unwrap();
                obj.hand_cards.push(card);
            }
        }

        // Put all cards left into the stock pile
        while let Some(n) = self.deck.cards.pop() {
            self.stock_pile.push(n);
        }
    }

    pub fn start_game(&mut self, game_id: u32, player_id: u32) -> Result<(), Error> {
        if self.host_id != player_id {
            return Err(Error::NotAHost);
        }

        if self.currently_playing {
            return Err(Error::RoomIsCurrentlyPlaying);
        }

        if self.players.len() < 3 {
            return Err(Error::PlayerNotEnough);
        }

        println!("Game dimulai");
        self.currently_playing = true;
        self.share_cards();

        if self.games.len() == 0 {
            let game = CardGame::new(self.config.clone());

            self.games.insert(game_id, game);
        }

        return Ok(());
    }

    pub fn insert_player(&mut self, player_id: u32) -> Result<(), String> {
        if self.players.len() >= 4 {
            return Err(String::from(
                "Dalam satu session hanya memuat maksimal 4 player",
            ));
        }
        let new_session_player = RoomPlayer::new();
        self.players.insert(player_id, new_session_player);
        self.player_turns.push(player_id);

        Ok(())
    }

    pub fn try_next_turn(&mut self) -> bool {
        if self.current_turn.is_complete() {
            if self.current_turn.index == self.players.len() - 1 {
                self.current_turn.index = 0;
            } else {
                self.current_turn.index += 1;
            }

            return true;
        }

        return false;
    }

    pub fn check_card_eligibility(
        card: &Card,
        player_card_hashset: &HashSet<&Card>,
        was_melding: bool,
    ) -> bool {
        let arr: Vec<SpotNumber> = SpotNumber::iter().collect();
        let court_type_iter: Vec<CourtType> = CourtType::iter().collect();

        match card.card_type {
            CardType::Spot(a) => {
                let mut s_number: Option<usize> = None;
                for (index, &sp) in arr.iter().enumerate() {
                    if sp == a {
                        s_number = Some(index);
                        break;
                    }
                }

                if s_number == None {
                    return false;
                }

                let s_number_unw = s_number.unwrap();

                let mut three_smaller = false;
                let mut two_smaller = false;
                let mut one_smaller = false;
                let mut one_greater = false;
                let mut two_greater = false;
                let mut three_greater = false;

                if s_number_unw >= 3 {
                    three_smaller = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*arr.get(s_number_unw - 3).unwrap()),
                    });
                }

                if s_number_unw >= 2 {
                    two_smaller = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*arr.get(s_number_unw - 2).unwrap()),
                    });
                }
                if s_number_unw >= 1 {
                    one_smaller = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*arr.get(s_number_unw - 1).unwrap()),
                    });
                }
                if s_number_unw <= 7 {
                    one_greater = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*arr.get(s_number_unw + 1).unwrap()),
                    });
                }
                if s_number_unw <= 6 {
                    two_greater = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*arr.get(s_number_unw + 2).unwrap()),
                    });
                }
                if s_number_unw <= 5 {
                    three_greater = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*arr.get(s_number_unw + 3).unwrap()),
                    });
                }

                if (two_smaller && one_smaller)
                    || (one_smaller && one_greater)
                    || (one_greater && two_greater)
                    || (three_smaller && two_smaller && one_smaller)
                    || (two_smaller && one_smaller && one_greater)
                    || (one_smaller && one_greater && two_greater)
                    || (one_greater && two_greater && three_greater)
                {
                    return true;
                }
            }
            CardType::Ace => {
                let mut player_ace_count = 0;
                for i in player_card_hashset {
                    if let CardType::Ace = i.card_type {
                        player_ace_count += 1;
                    }
                }

                if was_melding {
                    if player_ace_count >= 2 {
                        return true;
                    }
                } else {
                    if player_ace_count >= 3 {
                        return true;
                    }
                }
            }
            CardType::Court(a) => {
                let mut s_number: Option<usize> = None;
                for (index, &sp) in court_type_iter.iter().enumerate() {
                    if sp == a {
                        s_number = Some(index);
                        break;
                    }
                }

                if s_number == None {
                    return false;
                }

                let s_number_unw = s_number.unwrap();

                let mut two_smaller = false;
                let mut one_smaller = false;
                let mut one_greater = false;
                let mut two_greater = false;

                if s_number_unw >= 2 {
                    two_smaller = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Court(*court_type_iter.get(s_number_unw - 2).unwrap()),
                    });
                }
                if s_number_unw >= 1 {
                    one_smaller = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Court(*court_type_iter.get(s_number_unw - 1).unwrap()),
                    });
                }
                if s_number_unw <= 1 {
                    one_greater = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Court(*court_type_iter.get(s_number_unw + 1).unwrap()),
                    });
                }
                if s_number_unw <= 0 {
                    two_greater = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Court(*court_type_iter.get(s_number_unw + 2).unwrap()),
                    });
                }

                if (two_smaller && one_smaller)
                    || (one_smaller && one_greater)
                    || (one_greater && two_greater)
                {
                    return true;
                }
            }
            CardType::Joker(_) => {}
        }

        false
    }
}

/**
 *
 * The separate implementation below is for player's turn handler
 * Separated due to complexity
 *
 *
 *
 */

impl Room {
    pub async fn handle_draw_from_discard_pile(&mut self, player_id: u32) -> Result<Card, Error> {
        let max_draw = self.players.len() - 1;
        if &Some(DrawSource::StockPile) == &self.current_turn.draw_source {
            return Err(Error::RepeatTurn);
        }

        if let Some(arr) = &self.current_turn.drawn_card {
            if arr.len() > self.players.len() - 1 {
                return Err(Error::TooManyDraw);
            }
        }

        let mut pile = self.discard_pile.iter().rev().peekable();

        let player = self.players.get_mut(&player_id).unwrap();
        let player_card: HashSet<&Card> = player.hand_cards.iter().collect();

        for _ in 0..max_draw {
            if let Some(card) = pile.peek() {
                let is_eligible = Room::check_card_eligibility(
                    *card,
                    &player_card,
                    !player.melded_cards.is_empty(),
                );
                if !is_eligible {
                    return Err(Error::Ineligible);
                }
            }
            pile.next();
        }

        if let Some(card) = self.discard_pile.pop() {
            if let None = self.current_turn.drawn_card {
                self.current_turn.drawn_card = Some(Vec::new());
            }

            let current_turn_drawn_cards = self.current_turn.drawn_card.as_mut().unwrap();

            current_turn_drawn_cards.push(card);
            player.hand_cards.push(card);

            if self.current_turn.draw_source == None {
                self.current_turn.draw_source = Some(DrawSource::DiscardPile);
            }

            return Ok(card);
        } else {
            return Err(Error::CardNotFound);
        }
    }

    pub async fn handle_draw_from_stock_pile(&mut self, player_id: u32) -> Result<Card, Error> {
        let drawn_card = self.stock_pile.pop();
        let player = self.players.get_mut(&player_id).unwrap();

        if let Some(card) = drawn_card {
            self.current_turn.draw_source = Some(DrawSource::StockPile);
            self.current_turn.drawn_card = Some(vec![card]);
            player.hand_cards.push(card);
            return Ok(card);
        }

        Err(Error::CardNotFound)
    }

    pub fn handle_discard(&mut self, player_id: u32, card: Card) -> Result<Card, Error> {
        let player = self.players.get_mut(&player_id).unwrap();

        if card.card_type == CardType::Ace && player.melded_cards.is_empty() {
            return Err(Error::RequireMeld);
        }

        let card_index = player
            .hand_cards
            .iter()
            .position(|card_item| card_item == &card);

        if let Some(index) = card_index {
            let discarded = player.hand_cards.remove(index);
            self.current_turn.discarded_card = Some(discarded.clone());
            self.discard_pile.push(discarded);

            return Ok(card);
        }

        Err(Error::CardNotFound)
    }

    pub fn handle_meld(&mut self, player_id: u32, cards: Vec<Card>) -> Result<Vec<Card>, Error> {
        let player = self.players.get(&player_id).unwrap();
        let player_hand_cards_hs: HashSet<&Card> = player.hand_cards.iter().collect();

        for i in &cards {
            if !player_hand_cards_hs.contains(i) {
                return Err(Error::CardNotFound);
            }
        }

        let pivot = cards.get(0).unwrap();
        let cards_hs: HashSet<&Card> = cards.iter().collect();

        let res = Room::check_card_eligibility(pivot, &cards_hs, !player.melded_cards.is_empty());

        if res {
            return Ok(cards);
        }

        return Err(Error::Ineligible);
    }

    pub fn handle_put(&mut self, player_id: u32, cards: Vec<Card>) -> Result<Vec<Card>, Error> {
        let player = self.players.get_mut(&player_id).unwrap();

        let mut temp_hand_cards = player.hand_cards.clone();
        let mut cards_to_put: Vec<Card> = Vec::new();

        for i in &cards {
            if let Some(index) = temp_hand_cards.iter().position(|item| item == i) {
                let removed_card = temp_hand_cards.remove(index);
                cards_to_put.push(removed_card);
            } else {
                return Err(Error::CardNotFound);
            }
        }

        player.hand_cards = temp_hand_cards;
        player.putted_cards.extend(cards_to_put.clone());

        return Ok(cards_to_put);
    }
}
