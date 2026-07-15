use std::collections::HashMap;
use rand::RngExt;

use crate::{Room, RoomConfig};

#[derive(Debug)]
pub struct RoomManager {
    pub rooms: HashMap<[u8; 6], Room>,
    pub room_players: HashMap<u32, [u8; 6]>,
}

impl RoomManager {
    fn generate_room_id() -> [u8; 6] {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

        let mut rng = rand::rng();
        let mut result  = [0u8; 6];

        for i in 0..6 {
            let idx = rng.random_range(..CHARSET.len());
            result[i] = CHARSET[idx];
        }

        result
    }

    pub fn insert_room(&mut self, host_id: u32, cfg: RoomConfig) -> Result<[u8; 6], String> {
        if self.check_if_player_in_a_room(host_id) {
            return Err(format!(
                "[SESSION GAGAL DIBUAT] Player dengan ID {} sudah masuk di session",
                host_id
            ));
        }

        let new_session = Room::new(cfg, host_id);

        let room_id: [u8; 6];

        loop {
            let id = RoomManager::generate_room_id();
            if self.rooms.contains_key(&id) {
                continue;
            } else {
                room_id = id;
                break;
            }
        }

        match new_session {
            Ok(room) => {

                self.rooms.insert(room_id, room);
                println!("[SESSION DIBUAT] Id: {}", str::from_utf8(&room_id).unwrap());

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

    pub fn put_player_in_room(&mut self, player_id: u32, room_id: [u8; 6]) -> Result<(), String> {
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

        println!("Pemain {player_id} masuk room: {}", str::from_utf8(&room_id).unwrap());
        return Ok(());
    }

    pub fn get_room(&self, room_id: [u8; 6]) -> Option<&Room> {
        self.rooms.get(&room_id)
    }
}
