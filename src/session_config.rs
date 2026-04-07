use crate::{Card, CardType};

#[derive(Clone, Debug, Copy)]
pub struct SessionConfig {
    pub allow_court_stacking: bool,
    pub free_hit: bool,
    pub allow_railing: bool,
    pub with_joker: bool,
    pub joker_type: Option<Card>,
}

impl SessionConfig {
    pub fn new(
        allow_court_stacking: bool,
        free_hit: bool,
        allow_railing: bool,
        with_joker: bool,
        joker_type: Option<Card>,
    ) -> Result<Self, String> {
        // if player_count < 3 || player_count > 4 {
        //     return Err(String::from("Jumlah player antara 3 atau 4"));
        // }

        let mut cfg = Self {
            allow_court_stacking: allow_court_stacking,
            free_hit: free_hit,
            allow_railing: allow_railing,
            with_joker: with_joker,
            joker_type: None,
        };

        if with_joker {
            match joker_type {
                Some(n) => match n.card_type {
                    CardType::Spot(_) => {
                        cfg.joker_type = Some(n);
                    }
                    _ => return Err(String::from("Tipe joker harus angka biasa [Create Session Config gagal]")),
                },
                None => {}
            }
        }

        Ok(cfg)
    }
}
