use crate::{SessionConfig};

#[derive(Debug)]
pub struct CardGame {
    
    pub config_used: SessionConfig,
    pub dealer_id: u32,
    pub current_turn: u32
}

impl CardGame {
    pub fn new(cfg: SessionConfig, dealer_id: u32) -> Self {
        Self {
            config_used: cfg, 
            dealer_id: dealer_id, 
            current_turn: dealer_id 
        }
    }
}