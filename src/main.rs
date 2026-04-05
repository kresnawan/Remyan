use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone)]
enum CourtType {
    Jack,
    Queen,
    King,
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum JokerType {
    Red,
    Black,
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum CardIcon {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone)]
enum CardType {
    Ace,
    Court(CourtType),
    Spot(SpotNumber),
    Joker(JokerType),
}

#[derive(Debug, EnumIter, Clone)]
enum SpotNumber {
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
struct Card {
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
    fn get_card_power(&self) -> u32 {
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

enum PlayerStatus {
    Online,
    Away,
    Offline,
}

struct Player {
    player_id: u32,
    status: PlayerStatus,
}

struct SessionPlayer {
    player_id: u32,
}

struct SessionPlayerScoreLogRow {
    player_id: u32,
    score: i32,
}

struct GameSession {
    session_id: u32,
    games: Vec<CardGame>,
    players: Vec<SessionPlayer>,
    config: SessionConfig,
    session_admin_index: usize,
    player_score_table: Vec<SessionPlayerScoreLogRow>,
}

impl GameSession {
    pub fn new(session_id: u32, cfg: SessionConfig, host_id: u32) -> Self {
        let new_session_player = SessionPlayer { player_id: host_id };
        Self {
            session_id: session_id,
            games: vec![],
            players: vec![new_session_player],
            config: cfg,
            session_admin_index: 0,
            player_score_table: Vec::new(),
        }
    }
}

struct CardGame {
    kartu_guakan: Vec<Card>,
    bookie: Player,
}

#[derive(Clone)]
struct SessionConfig {
    boleh_timpa_jqk: bool,
    tutukan_bebas: bool,
    boleh_ngerail: bool,
    with_joker: bool,
    joker_type: Option<Card>,
}

impl SessionConfig {
    pub fn create(
        boleh_timpa_jqk: bool,
        tutukan_bebas: bool,
        boleh_ngerail: bool,
        with_joker: bool,
        joker_type: Option<Card>,
    ) -> Result<Self, String> {
        // if player_count < 3 || player_count > 4 {
        //     return Err(String::from("Jumlah player antara 3 atau 4"));
        // }

        let mut cfg = Self {
            boleh_timpa_jqk: boleh_timpa_jqk,
            tutukan_bebas: tutukan_bebas,
            boleh_ngerail: boleh_ngerail,
            with_joker: with_joker,
            joker_type: None,
        };

        if with_joker {
            match joker_type {
                Some(n) => match n.card_type {
                    CardType::Spot(_) => {
                        cfg.joker_type = Some(n);
                    }
                    _ => return Err(String::from("Tipe joker harus angka biasa")),
                },
                None => {}
            }
        }

        Ok(cfg)
    }
}

struct App {
    session_manager: SessionManager,
    players: HashMap<u32, Player>,
}

impl App {
    pub fn put_player_to_session(&mut self, player_id: u32, session_id: u32) -> Result<(), String> {
        if !self.players.contains_key(&player_id) {
            return Err(format!("Player dengan ID {} tidak ada", player_id));
        }

        if self.session_manager.check_if_player_in_a_session(player_id) {
            return Err(format!(
                "Player dengan ID {} sudah masuk di session",
                player_id
            ));
        }

        match self
            .session_manager
            .put_player_in_session(player_id, session_id)
        {
            Err(err) => {
                return Err(err);
            }

            Ok(_) => return Ok(()),
        }
    }
}

struct SessionManager {
    sessions: HashMap<u32, GameSession>,

    // <PlayerId, SessionId>
    session_players: HashMap<u32, u32>,
}

impl SessionManager {
    pub fn create_session(
        &mut self,
        session_id: u32,
        host_id: u32,
        cfg: SessionConfig,
    ) -> Result<(), String> {
        if self.check_if_player_in_a_session(host_id) {
            return Err(format!(
                "Gagal membuat sesi karena player {} masih di sesi lain",
                host_id
            ));
        }

        let new_session = GameSession::new(session_id, cfg, host_id);
        self.session_players.insert(host_id, session_id);
        self.sessions.insert(session_id, new_session);

        Ok(())
    }

    pub fn check_if_player_in_a_session(&self, player_id: u32) -> bool {
        self.session_players.contains_key(&player_id)
    }

    pub fn put_player_in_session(&mut self, player_id: u32, session_id: u32) -> Result<(), String> {
        let result = self.session_players.insert(player_id, session_id);

        match result {
            Some(_) => {
                return Ok(());
            }
            None => {
                return Err(format!(
                    "Terjadi kegagalan saat memasukkan player kedalam session"
                ));
            }
        }
    }
}

fn main() {
    let deck = Card::generate_deck(false);
    println!("{:#?}", deck);
    println!("{}", deck.len());
}
