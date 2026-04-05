use crate::{Card, CardType};

#[derive(Clone)]
pub struct SessionConfig {
    pub boleh_timpa_jqk: bool,
    pub tutukan_bebas: bool,
    pub boleh_ngerail: bool,
    pub with_joker: bool,
    pub joker_type: Option<Card>,
}

impl SessionConfig {
    pub fn create(
        boleh_timpa_jqk: bool,
        tutukan_bebas: bool,
        boleh_ngerail: bool,
        with_joker: bool,
        joker_type: Option<Card>,
    ) -> Result<Self, String> {
        // if player_count < 3 || player_count > 4 {
        //     return Err(String::from("Jumlah player antara 3 atau 4"));
        // }

        let mut cfg = Self {
            boleh_timpa_jqk: boleh_timpa_jqk,
            tutukan_bebas: tutukan_bebas,
            boleh_ngerail: boleh_ngerail,
            with_joker: with_joker,
            joker_type: None,
        };

        if with_joker {
            match joker_type {
                Some(n) => match n.card_type {
                    CardType::Spot(_) => {
                        cfg.joker_type = Some(n);
                    }
                    _ => return Err(String::from("Tipe joker harus angka biasa")),
                },
                None => {}
            }
        }

        Ok(cfg)
    }
}
