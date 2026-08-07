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

pub enum MeldNumber {
    Three,
    Four,
}

impl MeldNumber {
    pub fn as_number(&self) -> usize {
        match self {
            MeldNumber::Three => 3,
            MeldNumber::Four => 4,
        }
    }
}

#[derive(Debug)]
pub enum TurnType {
    IsHit,
    IsClosing,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ce_1() {
        assert_eq!(
            Room::check_card_eligibility_ex_2(
                &Card {
                    card_icon: Some(CardIcon::Spade),
                    card_type: CardType::Spot(SpotNumber::Seven),
                },
                &vec![
                    Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Six)),
                    Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Five)),
                ],
                true,
                false,
            ),
            Some(vec![
                Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Five)),
                Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Six)),
            ])
        );
    }

    #[test]
    fn ce_2() {
        assert_eq!(
            Room::check_card_eligibility_ex_2(
                &Card {
                    card_icon: Some(CardIcon::Spade),
                    card_type: CardType::Spot(SpotNumber::Six),
                },
                &vec![
                    Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Seven)),
                    Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Five)),
                ],
                true,
                false,
            ),
            Some(vec![
                Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Five)),
                Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Seven)),
            ])
        );
    }

    #[test]
    fn ce_3() {
        assert_eq!(
            Room::check_card_eligibility_ex_2(
                &Card {
                    card_icon: Some(CardIcon::Spade),
                    card_type: CardType::Spot(SpotNumber::Eight),
                },
                &vec![
                    Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Six)),
                    Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Five)),
                ],
                true,
                false,
            ),
            None
        );
    }

    #[test]
    fn ce_4() {
        assert_eq!(
            Room::check_card_eligibility_ex_2(
                &Card {
                    card_icon: Some(CardIcon::Spade),
                    card_type: CardType::Spot(SpotNumber::Four),
                },
                &vec![
                    Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Seven)),
                    Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Six)),
                    Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Five)),
                ],
                true,
                false,
            ),
            Some(vec![
                Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Five)),
                Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Six)),
                Card::new(Some(CardIcon::Spade), CardType::Spot(SpotNumber::Seven)),
            ])
        );
    }

    #[test]
    fn ce_5() {
        assert_eq!(
            Room::check_card_eligibility_ex_2(
                &Card::new(Some(CardIcon::Spade), CardType::Court(CourtType::King)),
                &vec![
                    Card::new(Some(CardIcon::Spade), CardType::Court(CourtType::Jack)),
                    Card::new(Some(CardIcon::Spade), CardType::Court(CourtType::Queen)),
                ],
                true,
                false,
            ),
            Some(vec![
                Card::new(Some(CardIcon::Spade), CardType::Court(CourtType::Jack)),
                Card::new(Some(CardIcon::Spade), CardType::Court(CourtType::Queen)),
            ])
        );
    }

    #[test]
    fn ce_6() {
        assert_eq!(
            Room::check_card_eligibility_ex_2(
                &Card::new(Some(CardIcon::Spade), CardType::Court(CourtType::Queen)),
                &vec![
                    Card::new(Some(CardIcon::Spade), CardType::Court(CourtType::Jack)),
                    Card::new(Some(CardIcon::Spade), CardType::Court(CourtType::King)),
                ],
                true,
                false,
            ),
            Some(vec![
                Card::new(Some(CardIcon::Spade), CardType::Court(CourtType::Jack)),
                Card::new(Some(CardIcon::Spade), CardType::Court(CourtType::King)),
            ])
        );
    }

    #[test]
    fn ce_7() {
        assert_eq!(
            Room::check_card_eligibility_ex_2(
                &Card::new(Some(CardIcon::Spade), CardType::Ace),
                &vec![
                    Card::new(Some(CardIcon::Heart), CardType::Ace),
                    Card::new(Some(CardIcon::Diamond), CardType::Ace),
                ],
                false,
                false,
            ),
            None
        );
    }

    #[test]
    fn ce_8() {
        assert_eq!(
            Room::check_card_eligibility_ex_2(
                &Card::new(Some(CardIcon::Spade), CardType::Ace),
                &vec![
                    Card::new(Some(CardIcon::Heart), CardType::Ace),
                    Card::new(Some(CardIcon::Diamond), CardType::Ace),
                ],
                true,
                false,
            ),
            Some(vec![
                Card::new(Some(CardIcon::Heart), CardType::Ace),
                Card::new(Some(CardIcon::Diamond), CardType::Ace),
            ])
        );
    }

    #[test]
    fn ce_9() {
        assert_eq!(
            Room::check_card_eligibility_ex_2(
                &Card::new(Some(CardIcon::Spade), CardType::Ace),
                &vec![
                    Card::new(Some(CardIcon::Heart), CardType::Ace),
                    Card::new(Some(CardIcon::Diamond), CardType::Ace),
                    Card::new(Some(CardIcon::Club), CardType::Ace),
                ],
                false,
                false,
            ),
            Some(vec![
                Card::new(Some(CardIcon::Heart), CardType::Ace),
                Card::new(Some(CardIcon::Diamond), CardType::Ace),
                Card::new(Some(CardIcon::Club), CardType::Ace),
            ])
        );
    }

    #[test]
    fn ce_10() {
        assert_eq!(
            Room::check_card_eligibility_ex_2(
                &Card::new(Some(CardIcon::Spade), CardType::Court(CourtType::Jack)),
                &vec![
                    Card::new(Some(CardIcon::Heart), CardType::Court(CourtType::Jack)),
                    Card::new(Some(CardIcon::Diamond), CardType::Court(CourtType::Jack)),
                    Card::new(Some(CardIcon::Club), CardType::Court(CourtType::Jack)),
                ],
                false,
                true,
            ),
            Some(vec![
                Card::new(Some(CardIcon::Heart), CardType::Court(CourtType::Jack)),
                Card::new(Some(CardIcon::Diamond), CardType::Court(CourtType::Jack)),
                Card::new(Some(CardIcon::Club), CardType::Court(CourtType::Jack)),
            ])
        );
    }
}


#[derive(Debug)]
pub struct Room {
    pub deck: Deck,
    pub stock_pile: Vec<Card>,
    pub discard_pile: Vec<(u32, Card)>,
    pub room_id: [u8; 6],
    pub games: HashMap<u32, CardGame>,
    pub players: HashMap<u32, RoomPlayer>,
    pub player_turns: Vec<u32>,
    pub config: RoomConfig,
    pub host_id: u32,
    pub currently_playing: bool,
    pub current_turn: PlayerTurn,
    pub counter: u32,
}

impl Room {
    pub fn new(cfg: RoomConfig, host_id: u32) -> Self {
        let new_session_player = RoomPlayer::new();
        let deck = Deck::new(cfg.joker.clone());

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
            counter: 0,
        }
    }

    fn current_turn_id(&self) -> u32 {
        self.player_turns[self.current_turn.index]
    }

    fn current_turn_player(&self) -> &RoomPlayer {
        self.players.get(&self.current_turn_id()).unwrap()
    }

    #[cfg(feature = "server")]
    fn share_cards(&mut self) {
        self.deck.shuffle();
        let hand_size = { if self.players.len() == 4 { 6 } else { 7 } };
        // Share cards
        for (_, player) in self.players.iter_mut() {
            for _ in 0..hand_size {
                let card = self.deck.cards.pop().unwrap();
                player.hand_cards.push(card);
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

    pub fn calculate_melded_cards(&mut self) {
        for (_, player) in &mut self.players {
            let flattened_melded: Vec<&Card> = player.melded_cards.iter().flatten().collect();

            for card in flattened_melded {
                player.current_score += card.get_meld_weight() as i32;
            }
        }
    }

    pub fn calculate_hand_cards(&mut self) {
        for id in &self.player_turns {
            let player = self.players.get_mut(id).unwrap();
            if player.hand_cards.is_empty() {
                continue;
            }

            let mut hand_cards = std::mem::take(&mut player.hand_cards);

            let mut unmeldable = Vec::new();
            let mut meldable = Vec::new();

            while let Some(pivot) = hand_cards.pop() {
                let elig = Room::check_card_eligibility_ex_2(
                    &pivot,
                    &hand_cards,
                    !player.melded_cards.is_empty(),
                    self.config.allow_railing,
                );

                if let Some(mut cards) = elig {
                    let mut indexed: Vec<usize> = cards
                        .iter()
                        .filter_map(|item| hand_cards.iter().position(|c| c == item))
                        .collect();

                    indexed.sort_by(|a, b| b.cmp(a));
                    indexed.iter().for_each(|ind| {
                        hand_cards.remove(*ind);
                    });

                    cards.push(pivot);
                    meldable.push(cards);
                } else {
                    unmeldable.push(pivot);
                }
            }

            let mut meldable_flatten: Vec<Card> = meldable.into_iter().flatten().collect();

            for i in &meldable_flatten {
                player.current_score += i.get_meld_weight() as i32;
            }

            for i in &unmeldable {
                player.current_score -= i.get_unmelded_weight() as i32;
            }

            unmeldable.append(&mut meldable_flatten);

            player.hand_cards = unmeldable;
        }
    }

    pub fn check_current_turn(&mut self) -> Option<TurnType> {
        if self.current_turn_player().hand_cards.is_empty() {
            if self.current_turn.draw_source == Some(DrawSource::DiscardPile) {
                self.current_turn.is_hit = true;
                let mut players_got_hit = Vec::new();

                if let Some(cards) = &self.current_turn.drawn_card {
                    for i in cards {
                        if let Some(pid) = i.0 {
                            players_got_hit.push(pid);
                        }
                    }
                }

                if let Some(card) = self.current_turn.discarded_card {
                    let hit_weight = card.get_hit_weight();

                    for i in players_got_hit {
                        let player = self.players.get_mut(&i).unwrap();
                        player.current_score -= hit_weight as i32;
                    }

                    if self.config.hitter_scoring {
                        let player = self.players.get_mut(&self.current_turn_id()).unwrap();
                        player.current_score += hit_weight as i32;
                    }
                }

                self.calculate_hand_cards();
                self.calculate_melded_cards();

                return Some(TurnType::IsHit);
            } else if self.current_turn.draw_source == Some(DrawSource::StockPile) {
                self.current_turn.is_closing = true;

                self.calculate_hand_cards();
                self.calculate_melded_cards();

                return Some(TurnType::IsClosing);
            }
        }

        return None;
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

    pub fn check_card_eligibility_ex(
        card: &Card,
        player_card_hashset: &HashSet<&Card>,
        was_melding: bool,
        allow_railing: bool,
    ) -> Option<MeldNumber> {
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
                return Some(MeldNumber::Four);
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
                {
                    return Some(MeldNumber::Three);
                } else if (three_smaller && two_smaller && one_smaller)
                    || (two_smaller && one_smaller && one_greater)
                    || (one_smaller && one_greater && two_greater)
                    || (one_greater && two_greater && three_greater)
                {
                    return Some(MeldNumber::Four);
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
                    if player_ace_count == 2 {
                        return Some(MeldNumber::Three);
                    } else {
                        return None;
                    }
                } else {
                    if player_ace_count == 3 {
                        return Some(MeldNumber::Four);
                    } else {
                        return None;
                    }
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
                    return Some(MeldNumber::Three);
                }
            }
            CardType::Joker(_) => {}
        }

        None
    }

    //////////////////////////////////////
    //
    // Because it returns the meld result as well, we assume that param `card` is outside player's hand
    //

    pub fn check_card_eligibility_ex_2(
        card: &Card,
        hand: &Vec<Card>,
        is_melding: bool,
        allow_railing: bool,
    ) -> Option<Vec<Card>> {
        if allow_railing {
            let current_icon_number = card.card_icon.unwrap().as_number();
            let mut is_eligible = true;

            let mut res_cards = Vec::new();

            for i in 1..=3 {
                let target = (current_icon_number + i) % 4;
                let equal_card = hand.iter().find(|item| {
                    item == &&Card::new(CardIcon::from_number(target), card.card_type)
                });

                if let Some(card) = equal_card {
                    res_cards.push(card.clone());
                    continue;
                } else {
                    is_eligible = false;
                    break;
                }
            }

            if is_eligible {
                return Some(res_cards);
            }
        }

        match card.card_type {
            CardType::Spot(_) => {
                let spot_number_iter: Vec<SpotNumber> = SpotNumber::iter().collect();
                let spot_index = card.get_spot_index().unwrap();
                let mut res_cards = Vec::new();

                let start_index = spot_index.saturating_sub(3);
                let end_index = std::cmp::min(spot_index + 3, spot_number_iter.len() - 1);

                for i in start_index..=end_index {
                    let sample_card = Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Spot(*spot_number_iter.get(i).unwrap()),
                    };

                    if &sample_card == card {
                        continue;
                    }

                    let found_card = hand.iter().find(|item| item == &&sample_card).or_else(|| {
                        hand.iter().find(|item| {
                            item == &&Card::joker(sample_card.get_color_type().unwrap())
                        })
                    });

                    if let Some(matched) = found_card {
                        if i >= spot_index + 2 && res_cards.is_empty() {
                            break;
                        } else {
                            res_cards.push(matched.clone());
                            if res_cards.len() == 3 {
                                break;
                            }
                        }
                    } else {
                        if res_cards.len() < 2 {
                            res_cards.clear();
                        } else {
                            if i < spot_index {
                                res_cards.clear();
                            }
                            break;
                        }
                    }
                }

                if res_cards.len() >= 2 {
                    return Some(res_cards);
                } else {
                    return None;
                }
            }
            CardType::Ace => {
                let mut res_cards = Vec::new();

                for i in hand {
                    if let CardType::Ace = i.card_type {
                        res_cards.push(i.clone());
                    }
                }

                if is_melding {
                    if res_cards.len() >= 2 {
                        return Some(res_cards);
                    }
                } else {
                    if res_cards.len() >= 3 {
                        return Some(res_cards);
                    }
                }

                return None;
            }
            CardType::Court(_) => {
                let court_type_iter: Vec<CourtType> = CourtType::iter().collect();
                let court_index = card.get_court_index().unwrap();
                let mut res_cards = Vec::new();

                let start_index = court_index.saturating_sub(2);
                let end_index = std::cmp::min(court_index + 2, court_type_iter.len() - 1);

                for i in start_index..=end_index {
                    let sample_card = Card {
                        card_icon: card.card_icon,
                        card_type: CardType::Court(*court_type_iter.get(i).unwrap()),
                    };

                    if &sample_card == card {
                        continue;
                    }

                    let found_card = hand.iter().find(|item| item == &&sample_card);

                    if let Some(matched) = found_card {
                        res_cards.push(matched.clone());
                        if res_cards.len() == 2 {
                            break;
                        }
                    } else {
                        if res_cards.len() < 2 {
                            res_cards.clear();
                        } else {
                            break;
                        }
                    }
                }

                if res_cards.len() >= 2 {
                    return Some(res_cards);
                } else {
                    return None;
                }
            }
            CardType::Joker(_) => {
                return None;
            }
        }
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

    pub fn get_players_hand(&mut self) -> Vec<(u32, Vec<Card>)> {
        let res: Vec<(u32, Vec<Card>)> = self
            .players
            .iter()
            .map(|item| (*item.0, item.1.hand_cards.clone()))
            .collect();

        res
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
    pub fn handle_draw_from_discard_pile(&mut self, player_id: u32) -> Result<(Card, bool), Error> {
        let mut is_interrupting = false;

        if !self.is_turn(player_id) {
            if self.config.free_hit {
                let player = self.players.get(&player_id).unwrap();
                let player_card_hashset = &player.hand_cards.iter().collect();

                let res = Room::check_card_eligibility_ex(
                    &self.discard_pile.last().unwrap().1,
                    player_card_hashset,
                    !player.melded_cards.is_empty(),
                    self.config.allow_railing,
                );

                if let Some(number) = res {
                    if number.as_number() != player.hand_cards.len() {
                        return Err(Error::Ineligible);
                    } else {
                        //
                        // Hit succeed
                        //
                        self.current_turn.reset();
                        let (ind, _) = self
                            .player_turns
                            .iter()
                            .enumerate()
                            .find(|(_, item)| **item == player_id)
                            .unwrap();
                        self.current_turn.index = ind;
                        is_interrupting = true;
                    }
                }
            } else {
                return Err(Error::FreeHitDisallowed);
            }
        }
        //
        // If player has drawn a card from stock pile before, return error
        //
        if &Some(DrawSource::StockPile) == &self.current_turn.draw_source {
            return Err(Error::RepeatTurn);
        }
        //
        // Set the max draw
        let mut max_draw = self.players.len() - 1;
        //
        // Evaluate the max draw
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
                    &card.1,
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
                vec.push((Some(card.0), card.1));
            } else {
                self.current_turn.drawn_card = Some(vec![(Some(card.0), card.1)])
            }

            player.hand_cards.push(card.1);

            self.current_turn.draw_source = Some(DrawSource::DiscardPile);

            return Ok((card.1, is_interrupting));
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

            self.current_turn.drawn_card = Some(vec![(None, card)]);

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
                if top_card.1.is_court() && card.is_court() {
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
            self.discard_pile.push((player_id, discarded));

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

        let res = Room::check_card_eligibility_ex(
            pivot.unwrap(),
            &cards_hs,
            !player.melded_cards.is_empty(),
            self.config.allow_railing,
        );

        if let Some(mn) = res {
            if self.player_turns[self.current_turn.index] == player_id {
                //
                // If someone draw a card from discard pile, they MUST meld
                // the card that been taken.
                // So it is not possible to take a card from discard pile without melding it
                //
                if let Some(DrawSource::DiscardPile) = self.current_turn.draw_source {
                    let mut contains_drawn_card = false;
                    for i in self.current_turn.drawn_card.as_ref().unwrap() {
                        if cards.contains(&i.1) {
                            contains_drawn_card = true;
                            break;
                        }
                    }

                    if !contains_drawn_card {
                        return Err(Error::DrawnCardRequired);
                    }
                }
                //
                // Check if this meld is player's last meld (resulting in empty hand).
                // If the drawn card's source was from stock pile, it is considered closing.
                // So far it covers some cases i've faced, so it doesn't guarantee
                // that this logic is completely reliable.
                //
                if mn.as_number() == player.hand_cards.len()
                    || (self.current_turn.drawn_card.is_some()
                        && mn.as_number() + 1 == player.hand_cards.len())
                {
                    if self.current_turn.draw_source == Some(DrawSource::StockPile)
                        && !self.config.allow_closing
                    {
                        return Err(Error::ClosingDisallowed);
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

            let mut indexes_to_remove: Vec<usize> = cards
                .iter()
                .filter_map(|item| player.hand_cards.iter().position(|ca| ca == item))
                .collect();

            indexes_to_remove.sort_by(|a, b| b.cmp(a));

            for i in indexes_to_remove {
                player.hand_cards.remove(i);
            }

            return Ok(cards);
        }

        return Err(Error::Ineligible);
    }
}
