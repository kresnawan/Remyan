use crate::{Card, CardType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NumberOfJokers {
    None,
    Two,
    Four,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoomConfig {
    pub allow_court_stacking: bool,
    pub free_hit: bool,
    pub allow_railing: bool,
    pub with_joker: bool,
    pub hitter_scoring: bool,
    pub allow_closing: bool,
    pub number_of_jokers: NumberOfJokers,
    pub joker_type: Option<Card>,
}

impl RoomConfig {
    pub fn new(
        allow_court_stacking: bool,
        free_hit: bool,
        allow_railing: bool,
        with_joker: bool,
        hitter_scoring: bool,
        allow_closing: bool,
        number_of_jokers: NumberOfJokers,
        joker_type: Option<Card>,
    ) -> Result<Self, String> {
        let mut cfg = Self {
            allow_court_stacking,
            free_hit,
            allow_railing,
            with_joker,
            allow_closing,
            hitter_scoring,
            number_of_jokers,
            joker_type,
        };

        if with_joker {
            match joker_type {
                Some(n) => match n.card_type {
                    CardType::Spot(_) => {
                        cfg.joker_type = Some(n);
                    }
                    _ => {
                        return Err(String::from(
                            "Tipe joker harus angka biasa [Create Session Config gagal]",
                        ));
                    }
                },
                None => {}
            }
        }

        Ok(cfg)
    }
}
