use crate::game::room_config::RoomConfig;

#[derive(Debug)]
pub struct CardGame {
    
    pub config_used: RoomConfig,
    pub current_dealer: u32,
    
}

impl CardGame {
    pub fn new(cfg: RoomConfig) -> Self {
        Self {
            config_used: cfg,
            current_dealer: 0
        }
    }
}