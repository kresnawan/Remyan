use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

mod config;
mod manager;
mod player;

pub use config::*;
pub use manager::*;
pub use player::*;

use crate::{
    Card, CardGame, CardIcon, CardType, CourtType, Deck, PlayerTurn, SpotNumber,
    protocol::{DrawSource, Error},
};

#[derive(Debug)]
pub struct Room {
    pub deck: Deck,
    pub stock_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub room_id: [u8; 6],
    pub games: HashMap<u32, CardGame>,
    pub players: HashMap<u32, RoomPlayer>,
    pub player_turns: Vec<u32>,
    pub config: RoomConfig,
    pub host_id: u32,
    pub currently_playing: bool,
    pub current_turn: PlayerTurn,
}

impl Room {
    pub fn new(cfg: RoomConfig, host_id: u32) -> Self {
        let new_session_player = RoomPlayer::new();
        let deck = Deck::new(true);

        let mut players = HashMap::new();
        players.insert(host_id, new_session_player);

        Self {
            deck: deck,
            stock_pile: Vec::new(),
            discard_pile: Vec::new(),
            room_id: [0u8; 6],
            games: HashMap::new(),
            players: players,
            player_turns: Vec::new(),
            config: cfg,
            host_id: host_id,
            currently_playing: false,
            current_turn: PlayerTurn::new(),
        }
    }

    #[cfg(feature = "server")]
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

    #[cfg(feature = "server")]
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

    pub fn insert_player(&mut self, player_id: u32) -> Result<(), Error> {
        if self.players.len() >= 4 {
            return Err(Error::TooManyPlayers);
        }
        let new_session_player = RoomPlayer::new();
        self.players.insert(player_id, new_session_player);
        self.player_turns.push(player_id);

        Ok(())
    }

    pub fn remove_player(&mut self, player_id: u32) -> Result<usize, String> {
        self.players.remove(&player_id).unwrap();
        let result: Vec<u32> = self
            .player_turns
            .clone()
            .into_iter()
            .filter(|&v| v != player_id)
            .collect();

        self.player_turns = result;

        if player_id == self.host_id {
            if self.player_turns.len() > 0 {
                self.host_id = self.player_turns[0]
            }
        }

        return Ok(self.player_turns.len());
    }

    pub fn try_next_turn(&mut self) -> Option<bool> {
        if self.current_turn.is_complete() {
            if self.current_turn.index == self.players.len() - 1 {
                self.current_turn.index = 0;
            } else {
                self.current_turn.index += 1;
            }

            if self.stock_pile.is_empty() {
                self.currently_playing = false;
                return None;
            }

            return Some(true);
        }

        return Some(false);
    }

    pub fn check_card_eligibility(
        card: &Card,
        player_card_hashset: &HashSet<&Card>,
        was_melding: bool,
        allow_railing: bool,
    ) -> bool {
        if allow_railing {
            let mut current_icon_number = card.card_icon.unwrap().as_number();
            let mut is_eligible = true;
            for i in 1..=3 {
                let target = (current_icon_number + i) % 4;
                let equal_card = player_card_hashset
                    .contains(&Card::new(CardIcon::from_number(target), card.card_type));

                if equal_card {
                    current_icon_number += 1;
                    continue;
                } else {
                    is_eligible = false;
                    break;
                }
            }

            if is_eligible {
                return true;
            }
        }

        match card.card_type {
            CardType::Spot(_) => {
                let spot_number_iter: Vec<SpotNumber> = SpotNumber::iter().collect();
                let spot_index = card.get_spot_index().unwrap();

                let mut three_smaller = false;
                let mut two_smaller = false;
                let mut one_smaller = false;
                let mut one_greater = false;
                let mut two_greater = false;
                let mut three_greater = false;

                if spot_index >= 3 {
                    let sample_card = Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*spot_number_iter.get(spot_index - 3).unwrap()),
                    };
                    three_smaller = player_card_hashset.contains(&sample_card)
                        || player_card_hashset
                            .contains(&Card::joker(sample_card.get_color_type().unwrap()));
                }

                if spot_index >= 2 {
                    let sample_card = Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*spot_number_iter.get(spot_index - 2).unwrap()),
                    };
                    two_smaller = player_card_hashset.contains(&sample_card)
                        || player_card_hashset
                            .contains(&Card::joker(sample_card.get_color_type().unwrap()));
                }
                if spot_index >= 1 {
                    let sample_card = Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*spot_number_iter.get(spot_index - 1).unwrap()),
                    };
                    one_smaller = player_card_hashset.contains(&sample_card)
                        || player_card_hashset
                            .contains(&Card::joker(sample_card.get_color_type().unwrap()));
                }
                if spot_index <= 7 {
                    let sample_card = Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*spot_number_iter.get(spot_index + 1).unwrap()),
                    };
                    one_greater = player_card_hashset.contains(&sample_card)
                        || player_card_hashset
                            .contains(&Card::joker(sample_card.get_color_type().unwrap()));
                }
                if spot_index <= 6 {
                    let sample_card = Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*spot_number_iter.get(spot_index + 2).unwrap()),
                    };
                    two_greater = player_card_hashset.contains(&sample_card)
                        || player_card_hashset
                            .contains(&Card::joker(sample_card.get_color_type().unwrap()));
                }
                if spot_index <= 5 {
                    let sample_card = Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*spot_number_iter.get(spot_index + 3).unwrap()),
                    };
                    three_greater = player_card_hashset.contains(&sample_card)
                        || player_card_hashset
                            .contains(&Card::joker(sample_card.get_color_type().unwrap()));
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
                    return player_ace_count >= 2;
                } else {
                    return player_ace_count >= 3;
                }
            }
            CardType::Court(_) => {
                let court_type_iter: Vec<CourtType> = CourtType::iter().collect();
                let court_index = card.get_court_index().unwrap();

                let mut two_smaller = false;
                let mut one_smaller = false;
                let mut one_greater = false;
                let mut two_greater = false;

                if court_index >= 2 {
                    two_smaller = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Court(*court_type_iter.get(court_index - 2).unwrap()),
                    });
                }
                if court_index >= 1 {
                    one_smaller = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Court(*court_type_iter.get(court_index - 1).unwrap()),
                    });
                }
                if court_index <= 1 {
                    one_greater = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Court(*court_type_iter.get(court_index + 1).unwrap()),
                    });
                }
                if court_index <= 0 {
                    two_greater = player_card_hashset.contains(&Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Court(*court_type_iter.get(court_index + 2).unwrap()),
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

    pub fn edit_config(&mut self, new_config: RoomConfig, player_id: u32) -> Result<(), Error> {
        if self.host_id != player_id {
            return Err(Error::NotAHost);
        }

        self.config = new_config;
        return Ok(());
    }

    fn is_turn(&self, player_id: u32) -> bool {
        self.player_turns[self.current_turn.index] == player_id
    }
}

//
//
// The separate implementation below is for player's turn handler
// Separated due to complexity
//
//
//
//

impl Room {
    pub fn handle_draw_from_discard_pile(&mut self, player_id: u32) -> Result<Card, Error> {
        if !self.is_turn(player_id) {
            return Err(Error::NotATurn);
        }
        //
        // If player has drawn a card from stock pile before, return error
        if &Some(DrawSource::StockPile) == &self.current_turn.draw_source {
            return Err(Error::RepeatTurn);
        }
        //
        // Set the max draw
        let mut max_draw = self.players.len() - 1;

        if let Some(cards) = &self.current_turn.drawn_card {
            max_draw -= cards.len();
        }

        if max_draw == 0 {
            return Err(Error::TooManyDraw);
        }

        let mut pile = self.discard_pile.iter().rev();

        let player = self.players.get_mut(&player_id).unwrap();
        let player_card: HashSet<&Card> = player.hand_cards.iter().collect();

        let mut is_eligible: bool = false;

        for _ in 0..max_draw {
            if let Some(card) = pile.next() {
                let is_elig = Room::check_card_eligibility(
                    card,
                    &player_card,
                    !player.melded_cards.is_empty(),
                    self.config.allow_railing,
                );

                if is_elig {
                    is_eligible = true;
                }
            }
        }

        if !is_eligible {
            return Err(Error::Ineligible);
        }

        if let Some(card) = self.discard_pile.pop() {
            if let Some(vec) = &mut self.current_turn.drawn_card {
                vec.push(card);
            } else {
                self.current_turn.drawn_card = Some(vec![card])
            }

            player.hand_cards.push(card);

            self.current_turn.draw_source = Some(DrawSource::DiscardPile);

            return Ok(card);
        } else {
            return Err(Error::CardNotFound);
        }
    }

    pub fn handle_draw_from_stock_pile(&mut self, player_id: u32) -> Result<Card, Error> {
        if !self.is_turn(player_id) {
            return Err(Error::NotATurn);
        }

        if let Some(_) = &self.current_turn.drawn_card {
            return Err(Error::RepeatTurn);
        }

        let player = self.players.get_mut(&player_id).unwrap();

        if let Some(card) = self.stock_pile.pop() {
            self.current_turn.draw_source = Some(DrawSource::StockPile);

            self.current_turn.drawn_card = Some(vec![card]);

            player.hand_cards.push(card);
            return Ok(card);
        }

        Err(Error::CardNotFound)
    }

    pub fn handle_discard(&mut self, player_id: u32, card: Card) -> Result<Card, Error> {
        if !self.is_turn(player_id) {
            return Err(Error::NotATurn);
        }
        //
        //
        // Consideration needed for this joker forbidement
        // since it could causing deadlock if player's left cards are all jokers
        if let CardType::Joker(_) = card.card_type {
            return Err(Error::DiscardAJoker);
        }

        let player = self.players.get_mut(&player_id).unwrap();

        if card.card_type == CardType::Ace && player.melded_cards.is_empty() {
            return Err(Error::RequireMeld);
        }

        if let Some(_) = self.current_turn.discarded_card {
            return Err(Error::RepeatTurn);
        }

        if !self.config.allow_court_stacking {
            let top_card = self.discard_pile.last();
            if let Some(top_card) = top_card {
                if top_card.is_court() && card.is_court() {
                    return Err(Error::InvalidCommand);
                }
            }
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

        let pivot = cards.iter().find(|item| !item.is_joker());

        if let None = pivot {
            return Err(Error::Ineligible);
        }

        let cards_hs: HashSet<&Card> = cards.iter().collect();

        let res = Room::check_card_eligibility(
            pivot.unwrap(),
            &cards_hs,
            !player.melded_cards.is_empty(),
            self.config.allow_railing,
        );

        if res {
            if self.player_turns[self.current_turn.index] == player_id {
                if let Some(DrawSource::DiscardPile) = self.current_turn.draw_source {
                    let mut contains_drawn_card = false;
                    for i in self.current_turn.drawn_card.as_ref().unwrap() {
                        if cards.contains(i) {
                            contains_drawn_card = true;
                            break;
                        }
                    }

                    if !contains_drawn_card {
                        return Err(Error::DrawnCardRequired);
                    }
                }
                if let Some(arr) = &mut self.current_turn.melded_card {
                    arr.append(&mut cards.clone());
                } else {
                    self.current_turn.melded_card = Some(cards.clone())
                }
            }

            let player = self.players.get_mut(&player_id).unwrap();
            player.melded_cards.push(cards.clone());

            println!("{:?}", player.melded_cards);
            
            return Ok(cards);
        }

        return Err(Error::Ineligible);
    }
}
