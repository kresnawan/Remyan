
use std::collections::HashMap;

use crate::game::{room::Room, room_config::RoomConfig};

#[derive(Debug)]
pub struct RoomManager {
    pub rooms: HashMap<u64, Room>,

    // <PlayerId, RoomId>
    pub room_players: HashMap<u32, u64>,
}

impl RoomManager {
    pub fn insert_room(
        &mut self,
        host_id: u32,
        cfg: RoomConfig,
    ) -> Result<u64, String> {

        if self.check_if_player_in_a_room(host_id) {
            return Err(format!(
                "[SESSION GAGAL DIBUAT] Player dengan ID {} sudah masuk di session",
                host_id
            ));
        }

        let new_session = Room::new(cfg, host_id);

        match new_session {
            Ok(room) => {
                let room_id = room.room_id;
                
                self.rooms.insert(room_id, room);
                println!("[SESSION DIBUAT] Id: {}", room_id);

                self.put_player_in_room(host_id, room_id).unwrap();

                Ok(room_id)
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub fn check_if_player_in_a_room(&self, player_id: u32) -> bool {
        self.room_players.contains_key(&player_id)
    }

    pub fn handle_room_game_start(&mut self, room_id: u64, game_id: u32) -> Result<(), String> {
        let session = self.rooms.get_mut(&room_id).unwrap();
        match session.start_new_game(game_id) {
            Ok(_) => {
                println!("[GAME DIMULAI] SessionId: {}, GameId: {}", room_id, game_id);
                return Ok(())
            },
            Err(err) => {
                return Err(err)
            }
        }
    }

    pub fn put_player_in_room(&mut self, player_id: u32, room_id: u64) -> Result<(), String> {
        if self.check_if_player_in_a_room(player_id) {
            return Err(format!(
                "Player dengan ID {} sudah masuk di session",
                player_id
            ));
        }

        let insert_session_players_result = self.room_players.insert(
            player_id, 
            room_id
        );
        let insert_session_player_result = self.rooms.get_mut(&room_id).unwrap().put_player_in_room(player_id);


        match insert_session_players_result {
            Some(_) => {
                return Err(format!(
                    "Terjadi kesalahan saat memasukkan player kedalam session"
                ));
            }
            None => {
                match insert_session_player_result {
                    Ok(_) => {
                        println!("[PLAYER MASUK SESSION] PlayerId: {}, SessionId: {}", player_id, room_id);
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
