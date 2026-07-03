use macroquad::prelude::*;

use crate::{
    page::Page,
    ui::{
        Object,
        button::{Button, ButtonConfig, regular_button::RegularButton},
        config::{dimension::Dimension, position::Position},
    },
};

pub struct MainMenu {
    player_name: String,
    objects: Vec<Box<dyn Object>>,
}

impl MainMenu {
    pub fn new(player_name: &str) -> MainMenu {
        let btn = RegularButton::new(
            Position::new(100.0, 100.0),
            Dimension::new(100.0, 50.0),
            ButtonConfig::new("Pencet", 24.0),
        )
        .on_click(|| println!("Terpencet"));

        return MainMenu {
            player_name: String::from(player_name),
            objects: vec![Box::new(btn)],
        };
    }
}

impl Page for MainMenu {
    fn update(&mut self) {
        for item in &mut self.objects {
            item.update();
        }
    }
    fn draw(&self) {
        clear_background(RED);
        for item in &self.objects {
            item.draw();
        }
    }
}
