use std::collections::HashMap;

use crate::{Card, CardGame, SessionConfig, Deck};

#[derive(Debug)]
pub struct Session {
    pub deck: Deck,
    pub stock_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub session_id: u32,
    pub games: HashMap<u32, CardGame>,
    pub players: Vec<SessionPlayer>,
    pub config: SessionConfig,
    pub session_host_index: usize,
    pub currently_playing: bool,
}

#[derive(Debug)]
pub struct SessionPlayer {
    pub player_id: u32,
    pub current_score: i32,
    pub card_stack: Vec<Card>
}

impl Session {
    pub fn new(session_id: u32, cfg: SessionConfig, host_id: u32) -> Result<Self, String> {
        let new_session_player = SessionPlayer { player_id: host_id, current_score: 0, card_stack: Vec::new() };
        let deck = Deck::new(cfg.with_joker);
        
        return Ok(Self {
            deck: deck,
            stock_pile: Vec::new(),
            discard_pile: Vec::new(),
            session_id: session_id,
            games: HashMap::new(),
            players: vec![new_session_player],
            config: cfg,
            session_host_index: 0,
            currently_playing: false,
        });
    }

    fn share_cards(&mut self, with_joker: bool) {
        self.deck.shuffle();
        // Share cards
        for i in &mut self.players {
            for _ in 0..6 {
                let card = self.deck.cards.pop().unwrap();
                i.card_stack.push(card);
            }
        }

        // Put all cards left into the stock pile
        for i in &self.deck.cards {
            self.stock_pile.push(*i);
        }
    }
    
    pub fn start_new_game(&mut self, game_id: u32) -> Result<(), String> {

        if self.currently_playing {
            return Err(format!("Sesi dengan ID {} saat ini sedang bermain [Start Game gagal]", self.session_id));
        }

        if self.players.len() < 3 {
            return Err(format!("Sesi dengan ID {} kekurangan setidaknya {} pemain [Start Game gagal]", self.session_id, 3 - self.players.len()));
        }

        println!("Game dimulai");
        self.currently_playing = true;

        self.share_cards(self.config.with_joker);

        if self.games.len() == 0 {
            let game = CardGame::new(self.config, self.players[self.session_host_index].player_id);
            
            self.games.insert(game_id, game);
        }


        // End game
        self.discard_pile = Vec::new();
        self.stock_pile = Vec::new();
        println!("Game telah selesai");
        self.currently_playing = false;
        return Ok(());
        
    }

    pub fn put_player_in_session(&mut self, player_id: u32) -> Result<(), String> {
        if self.players.len() > 4 {
            return Err(String::from("Dalam satu session hanya memuat maksimal 4 player [Put Player in Session gagal]"));
        }
        let new_session_player = SessionPlayer {player_id: player_id, current_score: 0, card_stack: Vec::new()};
        self.players.push(new_session_player);

        Ok(())
    }
}
