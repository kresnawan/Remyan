use macroquad::prelude::*;

use crate::{
    PageIndex,
    page::Page,
    ui::{
        Object,
        button::{Button, ButtonConfig, regular_button::RegularButton},
        config::position::Position,
        container::Container,
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
                .set_dimensions(screen_width() / 2.0, 0.0);
        let join_room_btn = RegularButton::new(
            Position::new(0.0, 200.0),
            ButtonConfig::default("Masuk Room"),
        )
        .on_click(|| {
            println!("Masuk Room");
            return Some(PageIndex::Room as usize);
        })
        .set_padding(0.0, 50.0)
        .set_dimensions(screen_width() / 2.0, 0.0);
        let settings_btn = RegularButton::new(
            Position::new(0.0, 200.0 * 2.0),
            ButtonConfig::default("Pengaturan"),
        )
        .on_click(|| {
            println!("Masuk Room");
            return Some(PageIndex::Room as usize);
        })
        .set_padding(0.0, 50.0)
        .set_dimensions(screen_width() / 2.0, 0.0);

        let div = Container::new(0.0, 100.0, screen_width(), screen_height(), true, true)
            .add_child(Box::new(create_room_btn))
            .add_child(Box::new(join_room_btn))
            .add_child(Box::new(settings_btn));

        let div_2 = Container::new(0.0, 100.0, 700.0, 300.0, false, false).add_child(Box::new(div));

        return MainMenu {
            player_name: String::from(player_name),
            objects: vec![Box::new(div_2)],
        };
    }
}

impl Page for MainMenu {
    fn update(&mut self) -> Option<usize> {
        for item in &mut self.objects {
            if let Some(n) = item.update(None, None, None, None) {
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
