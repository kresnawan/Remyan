use macroquad::{color, prelude::*};
use remyan_core::App;

use crate::{
    PageIndex,
    page::Page,
    ui::{
        Object,
        button::{Button, ButtonConfig, regular_button::RegularButton},
        config::{dimension::Dimension, position::Position},
        font::Nunito,
    },
};

pub struct MainMenu {
    player_name: String,
    objects: Vec<Box<dyn Object>>,
}

impl MainMenu {
    pub fn new(player_name: &str) -> MainMenu {
        let btn_1 = RegularButton::new(
            Position::new(0.0, 0.0),
            ButtonConfig::new(
                "Buat Room",
                48.0,
                color::ORANGE,
                color::WHITE,
                Nunito::regular(),
            ),
        )
        .on_click(|| {
            println!("Terpencet");
            return None;
        })
        .set_padding(100.0, 50.0)
        .to_center_x();

        let btn_2 = RegularButton::new(
            Position::new(0.0, 40.0),
            ButtonConfig::new(
                "Masuk Room",
                48.0,
                color::ORANGE,
                color::WHITE,
                Nunito::regular(),
            ),
        )
        .on_click(|| {
            println!("Masuk Room");
            return Some(PageIndex::Room as usize);
        })
        .set_padding(100.0, 50.0)
        .to_center_x();

        return MainMenu {
            player_name: String::from(player_name),
            objects: vec![Box::new(btn_1), Box::new(btn_2)],
        };
    }
}

impl Page for MainMenu {
    fn update(&mut self) -> Option<usize> {
        for item in &mut self.objects {
            if let Some(n) = item.update() {
                return Some(n);
            }
        }

        return None;
    }
    fn draw(&self) {
        clear_background(RED);
        for item in &self.objects {
            item.draw();
        }
    }
}
