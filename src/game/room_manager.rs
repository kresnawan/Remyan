use std::collections::HashMap;

use crate::game::{room::Room, room_config::RoomConfig};

#[derive(Debug)]
pub struct RoomManager {
    pub rooms: HashMap<u64, Room>,

    // <PlayerId, RoomId>
    pub room_players: HashMap<u32, u64>,
}

impl RoomManager {
    pub fn insert_room(&mut self, host_id: u32, cfg: RoomConfig) -> Result<u64, String> {
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

    pub fn put_player_in_room(&mut self, player_id: u32, room_id: u64) -> Result<(), String> {
        if self.check_if_player_in_a_room(player_id) {
            return Err(format!("Pemain {player_id} sudah masuk ke room"));
        }

        self.room_players.insert(player_id, room_id);

        let room = match self.rooms.get_mut(&room_id) {
            Some(r) => r,
            None => {
                self.room_players.remove(&player_id);
                return Err(String::from("Room tidak ditemukan"));
            }
        };

        if let Err(e) = room.insert_player(player_id) {
            self.room_players.remove(&player_id);
            return Err(e);
        };

        println!("Pemain {player_id} masuk room: {room_id}");
        return Ok(());
    }

    pub fn get_room(&self, room_id: u64) -> Option<&Room> {
        self.rooms.get(&room_id)
    }
}
