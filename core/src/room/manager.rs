use std::collections::HashMap;

#[cfg(feature = "server")]
use rand::RngExt;

#[cfg(feature = "server")]
use crate::protocol::Error;
use crate::{Room, RoomConfig};

#[derive(Debug)]
pub struct RoomManager {
    pub rooms: HashMap<[u8; 6], Room>,
    pub room_players: HashMap<u32, [u8; 6]>,
}

impl RoomManager {
    #[cfg(feature = "server")]
    fn generate_room_id() -> [u8; 6] {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

        let mut rng = rand::rng();
        let mut result = [0u8; 6];

        for i in 0..6 {
            let idx = rng.random_range(..CHARSET.len());
            result[i] = CHARSET[idx];
        }

        result
    }

    #[cfg(feature = "server")]
    pub fn generate_player_id() -> u32 {
        rand::rng().random()
    }

    #[cfg(feature = "server")]
    pub fn insert_room(&mut self, host_id: u32, cfg: RoomConfig) -> Result<[u8; 6], Error> {
        if self.check_if_player_in_a_room(host_id) {
            return Err(Error::AlreadyJoined);
        }

        let room = Room::new(cfg, host_id);

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

        self.rooms.insert(room_id, room);
        println!("[SESSION DIBUAT] Id: {}", str::from_utf8(&room_id).unwrap());

        self.join_room(host_id, room_id).unwrap();
        Ok(room_id)
    }

    pub fn check_if_player_in_a_room(&self, player_id: u32) -> bool {
        self.room_players.contains_key(&player_id)
    }

    #[cfg(feature = "server")]
    pub fn join_room(&mut self, player_id: u32, room_id: [u8; 6]) -> Result<(), Error> {
        if self.check_if_player_in_a_room(player_id) {
            return Err(Error::AlreadyJoined);
        }

        self.room_players.insert(player_id, room_id);

        let room = match self.rooms.get_mut(&room_id) {
            Some(r) => r,
            None => {
                self.room_players.remove(&player_id);
                return Err(Error::RoomNotFound);
            }
        };

        if let Err(e) = room.insert_player(player_id) {
            self.room_players.remove(&player_id);
            return Err(e);
        };

        println!(
            "Pemain {player_id} masuk room: {}",
            str::from_utf8(&room_id).unwrap()
        );
        return Ok(());
    }

    pub fn remove_player_from_room(&mut self, player_id: u32) -> Result<usize, String> {
        if let Some(v) = self.room_players.remove(&player_id) {
            let room = self.rooms.get_mut(&v).unwrap();
            let room_player_count = room.remove_player(player_id);

            if let Ok(len) = room_player_count {
                if len == 0 {
                    self.rooms.remove(&v).unwrap();
                }

                return Ok(len);
            } else if let Err(err) = room_player_count {
                return Err(err);
            }
        }

        return Err(format!("Player tidak ditemukan"));
    }

    pub fn get_room(&self, room_id: [u8; 6]) -> Option<&Room> {
        self.rooms.get(&room_id)
    }
}
