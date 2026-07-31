use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NumberOfJokers {
    Two,
    Four,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomConfig {
    pub allow_court_stacking: bool,
    pub free_hit: bool,
    pub allow_railing: bool,
    pub joker: Option<NumberOfJokers>,
    pub hitter_scoring: bool,
    pub allow_closing: bool,
}

impl RoomConfig {
    pub fn new(
        allow_court_stacking: bool,
        free_hit: bool,
        allow_railing: bool,
        joker: Option<NumberOfJokers>,
        hitter_scoring: bool,
        allow_closing: bool,
    ) -> Result<Self, String> {
        let cfg = Self {
            allow_court_stacking,
            free_hit,
            allow_railing,
            allow_closing,
            hitter_scoring,
            joker,
        };

        Ok(cfg)
    }

    pub fn default() -> Self {
        Self {
            allow_court_stacking: false,
            free_hit: false,
            allow_railing: false,
            joker: None,
            hitter_scoring: false,
            allow_closing: false,
        }
    }
}
