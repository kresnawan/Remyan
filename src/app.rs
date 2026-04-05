use std::collections::HashMap;

use crate::{Player, SessionConfig, SessionManager};

pub struct App {
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
