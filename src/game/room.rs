use rand::RngExt;
use std::collections::HashMap;

use crate::{
    Tx,
    game::{card::Card, card_game::CardGame, deck::Deck, room_config::RoomConfig},
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

#[derive(Debug)]
pub struct RoomPlayer {
    pub current_score: i32,
    pub card_stack: Vec<Card>,
    pub tx: Option<Tx>,
}

impl Room {
    pub fn new(cfg: RoomConfig, host_id: u32) -> Result<Self, String> {
        let new_session_player = RoomPlayer {
            current_score: 0,
            card_stack: Vec::new(),
            tx: None,
        };
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
        for (&player_id, obj) in self.players.iter_mut() {
            for _ in 0..6 {
                let card = self.deck.cards.pop().unwrap();
                obj.card_stack.push(card);
            }
        }

        // Put all cards left into the stock pile
        while let Some(n) = self.deck.cards.pop() {
            self.stock_pile.push(n);
        }
    }

    pub fn start_game(&mut self, game_id: u32) -> Result<(), String> {
        // Start
        if self.currently_playing {
            return Err(format!(
                "[START GAME GAGAL] Sesi dengan ID {} saat ini sedang bermain",
                self.room_id
            ));
        }

        if self.players.len() < 3 {
            return Err(format!(
                "[START GAME GAGAL] Sesi dengan ID {} kekurangan setidaknya {} pemain",
                self.room_id,
                3 - self.players.len()
            ));
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
        let new_session_player = RoomPlayer {
            current_score: 0,
            card_stack: Vec::new(),
            tx: None,
        };
        self.players.insert(player_id, new_session_player);
        self.player_turns.push(player_id);

        Ok(())
    }

    fn get_room_player_mut(&mut self, player_id: u32) -> Option<&mut RoomPlayer> {
        self.players.get_mut(&player_id)
    }

    fn get_room_player(&self, player_id: u32) -> Option<&RoomPlayer> {
        self.players.get(&player_id)
    }

    fn handle_draw_player_card(&mut self, player_id: u32, draw_source: u8) {
        if draw_source == 0 {
            let card = self.stock_pile.pop().unwrap();
            self.get_room_player_mut(player_id)
                .unwrap()
                .card_stack
                .push(card);
        } else if draw_source == 1 {
            let mut current_discard_stack = self.discard_pile.iter().rev();
            for i in 0..self.players.len() {
                let item = current_discard_stack.next();
                // Handle draws from discard pile
            }
        }
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
}
