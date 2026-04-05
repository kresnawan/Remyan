use crate::{Session, SessionConfig};
use std::collections::HashMap;

pub struct SessionManager {
    pub sessions: HashMap<u32, Session>,

    // <PlayerId, SessionId>
    pub session_players: HashMap<u32, u32>,
}

impl SessionManager {
    pub fn create_session(
        &mut self,
        session_id: u32,
        host_id: u32,
        cfg: SessionConfig,
    ) -> Result<(), String> {
        let new_session = Session::new(session_id, cfg, host_id);

        match new_session {
            Ok(n) => {
                self.session_players.insert(host_id, session_id);
                self.sessions.insert(session_id, n);

                Ok(())
            }
            Err(err) => {
                return Err(err);
            }
        }
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
