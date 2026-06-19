use axum::extract::ws::{Message, Utf8Bytes};
use rand::RngExt;
use std::{
    collections::{HashMap, HashSet}
};
use strum::IntoEnumIterator;

use crate::{
    game::{
        card::{Card, CardType, CourtType, SpotNumber},
        card_game::CardGame,
        deck::Deck,
        room_config::RoomConfig, room_player::RoomPlayer,
    },
    network::ws::token::event::{Error::{self, PlayerNotEnough, RoomIsCurrentlyPlaying}, EventToken, ServerEvent},
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
    pub current_turn: usize,
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
            current_turn: 0,
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

    pub fn start_game(&mut self, game_id: u32) -> Result<(), Error> {
        // Start
        if self.currently_playing {
            return Err(RoomIsCurrentlyPlaying);
        }

        if self.players.len() < 3 {
            return Err(PlayerNotEnough);
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

    fn get_room_player(&self, player_id: u32) -> Option<&RoomPlayer> {
        self.players.get(&player_id)
    }

    fn next_turn(&mut self) {
        if self.current_turn == self.players.len() - 1 {
            self.current_turn = 0;
        } else {
            self.current_turn += 1;
        }
    }

    pub fn handle_turn(&mut self, player_id: u32) -> Result<(), String> {
        if let Some(_) = self.get_room_player(player_id) {
            if self.player_turns[self.current_turn] == player_id {
                // Handle player's turn
                self.stock_pile.pop().unwrap();

                self.next_turn();
                return Ok(());
            } else {
                return Err(format!("[TURN GAGAL] PlayerId belum saatnya bermain"));
            }
        } else {
            return Err(format!("[TURN GAGAL] Player tidak ditemukan di room"));
        }
    }

    pub fn broadcast(&self, token: EventToken, player_id: u32, all: bool) -> Result<(), String> {
        let serialized = serde_json::to_string(&token).unwrap();
        let payload = Utf8Bytes::from(serialized);
        for (&pid, pd) in self.players.iter() {
            if all || pid != player_id {
                let ptx = match pd.tx.as_ref() {
                    Some(tx) => tx,
                    None => {
                        println!("broadcast: terjadi error pada tx.as_ref()");
                        continue;
                    }
                };

                if let Err(_) = ptx.send(Message::Text(payload.clone())) {
                    println!("broadcast: terjadi error saat mengirim ke pesan websocket ke client");
                    continue;
                }
            }
        }

        Ok(())
    }

    pub fn broadcast_card(&self) -> Result<(), String> {
        for (_, pd) in self.players.iter() {
            let ptx = match pd.tx.as_ref() {
                Some(tx) => tx,
                None => {
                    println!("broadcast_card: terjadi error pada tx.as_ref()");
                    continue;
                }
            };

            let token = EventToken::ServerEvent(ServerEvent::PlayerCard(pd.hand_cards.clone()));
            let serialized = serde_json::to_string(&token).unwrap();
            let payload = Utf8Bytes::from(serialized);

            if let Err(_) = ptx.send(Message::Text(payload.clone())) {
                println!(
                    "broadcast_card: terjadi error saat mengirim ke pesan websocket ke client"
                );
                continue;
            }
        }

        Ok(())
    }

    pub fn ws_send_player(&self, token: EventToken, player_id: u32) -> Result<(), String> {
        let player = self.players.get(&player_id).unwrap();
        let serialized = serde_json::to_string(&token).unwrap();
        let payload = Utf8Bytes::from(serialized);

        if let Some(ptx) = player.tx.as_ref() {
            match ptx.send(Message::Text(payload.clone())) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e.to_string()),
            }
        }

        Err(String::from(
            "ws_send_player: terjadi error pada tx.as_ref()",
        ))
    }

    pub fn handle_draw_from_discard_pile(&mut self, number: usize, player_id: u32) {
        let mut pile = self.discard_pile.iter().peekable();
        let mut candidate: Vec<&Card> = Vec::new();

        for i in 0..number {
            if let Some(&card) = pile.peek() {
                candidate.push(card);
            }
        }
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

                let mut two_smaller = false;
                let mut one_smaller = false;
                let mut one_greater = false;
                let mut two_greater = false;

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

                if (two_smaller && one_smaller)
                    || (one_smaller && one_greater)
                    || (one_greater && two_greater)
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
