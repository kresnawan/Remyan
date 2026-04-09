use crate::{Session, SessionConfig};
use std::collections::HashMap;

#[derive(Debug)]
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
        if self.sessions.contains_key(&session_id) {
            return Err(format!("[SESSION GAGAL DIBUAT] ID session {} telah dipakai", session_id));
        }

        if self.check_if_player_in_a_session(host_id) {
            return Err(format!(
                "[SESSION GAGAL DIBUAT] Player dengan ID {} sudah masuk di session",
                host_id
            ));
        }

        let new_session = Session::new(session_id, cfg, host_id);

        match new_session {
            Ok(n) => {
                self.session_players.insert(host_id, session_id);
                self.sessions.insert(session_id, n);

                println!("[SESSION DIBUAT] Id: {}", session_id);

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

    pub fn handle_session_game_start(&mut self, session_id: u32, game_id: u32) -> Result<(), String> {
        let session = self.sessions.get_mut(&session_id).unwrap();
        match session.start_new_game(game_id) {
            Ok(_) => {
                println!("[GAME DIMULAI] SessionId: {}, GameId: {}", session_id, game_id);
                return Ok(())
            },
            Err(err) => {
                return Err(err)
            }
        }
    }

    pub fn put_player_in_session(&mut self, player_id: u32, session_id: u32) -> Result<(), String> {
        if self.check_if_player_in_a_session(player_id) {
            return Err(format!(
                "Player dengan ID {} sudah masuk di session",
                player_id
            ));
        }

        let insert_session_players_result = self.session_players.insert(
            player_id, 
            session_id
        );
        let insert_session_player_result = self.sessions.get_mut(&session_id).unwrap().put_player_in_session(player_id);


        match insert_session_players_result {
            Some(_) => {
                return Err(format!(
                    "Terjadi kesalahan saat memasukkan player kedalam session"
                ));
            }
            None => {
                match insert_session_player_result {
                    Ok(_) => {
                        println!("[PLAYER MASUK SESSION] PlayerId: {}, SessionId: {}", player_id, session_id);
                        return Ok(());
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
                
            }
        }
    }
}
