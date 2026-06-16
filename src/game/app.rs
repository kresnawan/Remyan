use std::collections::HashMap;

use crate::game::player::{Player, PlayerStatus};
use crate::game::room_config::RoomConfig;
use crate::game::room_manager::RoomManager;

pub struct App {
    pub room_manager: RoomManager,
    pub players: HashMap<u32, Player>,
}

impl App {
    pub fn new() -> Self {
        return Self {
            room_manager: RoomManager {
                rooms: HashMap::new(),
                room_players: HashMap::new(),
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

                println!("[PLAYER TEREGISTRASI] PlayerId: {}", id);

                return Ok(());
            }
        }
    }

    pub fn get_player_by_id(&mut self, player_id: u32) -> Option<&mut Player> {
        self.players.get_mut(&player_id)
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

    pub fn put_player_to_room(&mut self, player_id: u32, room_id: u64) -> Result<(), String> {
        // match self.get_player_status(player_id) {
        //     Some(n) => match n {
        //         PlayerStatus::Offline => {
        //             return Err(format!("Player dengan id {} sedang offline", player_id));
        //         }
        //         _ => {}
        //     },
        //     None => return Err(format!("Player dengan id {} tidak terdaftar", player_id)),
        // }

        if let Err(e) = self.room_manager.put_player_in_room(player_id, room_id) {
            return Err(e);
        }

        return Ok(());
    }

    pub fn handle_login(&mut self, player_id: u32) -> Result<(), String> {
        match self.get_player_status(player_id) {
            Some(n) => match n {
                PlayerStatus::Online => {
                    return Err(format!("Player dengan ID {} telah login", player_id));
                }
                _ => {}
            },
            None => {
                return Err(format!("Player dengan ID {} belum teregistrasi", player_id));
            }
        }

        self.get_player_by_id(player_id).unwrap().status = PlayerStatus::Online;
        Ok(())
    }

    pub fn create_room(&mut self, host_id: u32, cfg: RoomConfig) -> Result<u64, String> {
        // match self.get_player_status(host_id) {
        //     Some(n) => match n {
        //         PlayerStatus::Offline => {
        //             return Err(format!("[ROOM GAGAL DIBUAT] Player dengan ID {} sedang offline", host_id));
        //         }
        //         _ => {}
        //     },
        //     None => return Err(format!("[ROOM GAGAL DIBUAT] Player dengan ID {} belum teregistrasi", host_id)),
        // }

        self.room_manager.insert_room(host_id, cfg)
    }
}
