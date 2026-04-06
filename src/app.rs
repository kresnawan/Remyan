use std::collections::HashMap;

use crate::{Player, SessionConfig, SessionManager, player::PlayerStatus};

pub struct App {
    pub session_manager: SessionManager,
    pub players: HashMap<u32, Player>,
}

impl App {
    pub fn new() -> Self {
        return Self {
            session_manager: SessionManager {
                sessions: HashMap::new(),
                session_players: HashMap::new(),
            },
            players: HashMap::new(),
        };
    }

    pub fn register_new_player(&mut self, id: u32, uname: String) -> Result<(), String> {
        match self.get_player_status(id) {
            Some(_) => {
                return Err(format!(
                    "Player dengan ID {} telah terdaftar, setiap ID harus unik",
                    id
                ));
            }
            None => {
                let new_player = Player::new(uname);
                self.players.insert(id, new_player);

                return Ok(());
            }
        }
    }

    pub fn get_player_status(&self, player_id: u32) -> Option<PlayerStatus> {
        match self.players.get(&player_id) {
            Some(n) => {
                return Some(n.status);
            }
            None => {
                return None;
            }
        }
    }
    pub fn put_player_to_session(&mut self, player_id: u32, session_id: u32) -> Result<(), String> {
        match self.get_player_status(player_id) {
            Some(n) => match n {
                PlayerStatus::Offline => {
                    return Err(format!("Player dengan id {} sedang offline", player_id));
                }
                _ => {}
            },
            None => return Err(format!("Player dengan id {} tidak terdaftar", player_id)),
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

    pub fn create_session(
        &mut self,
        session_id: u32,
        host_id: u32,
        cfg: SessionConfig,
    ) -> Result<(), String> {
        if !self.players.contains_key(&host_id) {
            return Err(format!("Player dengan ID {} tidak ada", host_id));
        }

        if self.session_manager.check_if_player_in_a_session(host_id) {
            return Err(format!(
                "Player dengan ID {} sudah masuk di session",
                host_id
            ));
        }

        let res = self
            .session_manager
            .create_session(session_id, host_id, cfg);
        match res {
            Ok(()) => {
                return Ok(());
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
}
