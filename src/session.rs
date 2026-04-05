use crate::{CardGame, SessionConfig};

pub struct Session {
    pub session_id: u32,
    pub games: Vec<CardGame>,
    pub players: Vec<SessionPlayer>,
    pub config: SessionConfig,
    pub session_admin_index: usize,
    pub player_score_table: Vec<SessionPlayerScoreLogRow>,
    pub currently_playing: bool,
}

pub struct SessionPlayer {
    pub player_id: u32,
}

pub struct SessionPlayerScoreLogRow {
    pub player_id: u32,
    pub score: i32,
}

impl Session {
    pub fn new(session_id: u32, cfg: SessionConfig, host_id: u32) -> Result<Self, String> {
        let new_session_player = SessionPlayer { player_id: host_id };
        return Ok(Self {
            session_id: session_id,
            games: vec![],
            players: vec![new_session_player],
            config: cfg,
            session_admin_index: 0,
            player_score_table: Vec::new(),
            currently_playing: false,
        });
    }

    pub fn start_game(&mut self) {
        self.currently_playing = true;
    }
}
