use macroquad::{color, prelude::*};

use crate::{
    PageIndex,
    page::Page,
    ui::{
        Object,
        button::{Button, ButtonConfig, regular_button::RegularButton},
        config::position::Position,
        font::Nunito,
    },
};

pub struct MainMenu {
    player_name: String,
    objects: Vec<Box<dyn Object>>,
}

impl MainMenu {
    pub fn new(player_name: &str) -> MainMenu {
        let create_room_btn =
            RegularButton::new(Position::new(0.0, 0.0), ButtonConfig::default("Buat Room"))
                .on_click(|| {
                    println!("Terpencet");
                    return None;
                })
                .set_padding(0.0, 50.0)
                .set_dimensions(screen_width() / 2.0, 0.0)
                .to_center_x();

        let join_room_btn = RegularButton::new(
            Position::new(0.0, 200.0),
            ButtonConfig::default("Masuk Room"),
        )
        .on_click(|| {
            println!("Masuk Room");
            return Some(PageIndex::Room as usize);
        })
        .set_padding(0.0, 50.0)
        .set_dimensions(screen_width() / 2.0, 0.0)
        .to_center_x();

        let settings_btn = RegularButton::new(
            Position::new(0.0, 200.0 * 2.0),
            ButtonConfig::default("Pengaturan"),
        )
        .on_click(|| {
            println!("Masuk Room");
            return Some(PageIndex::Room as usize);
        })
        .set_padding(0.0, 50.0)
        .set_dimensions(screen_width() / 2.0, 0.0)
        .to_center_x();

        return MainMenu {
            player_name: String::from(player_name),
            objects: vec![
                Box::new(create_room_btn),
                Box::new(join_room_btn),
                Box::new(settings_btn),
            ],
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
