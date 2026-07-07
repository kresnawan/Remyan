use macroquad::prelude::*;

use crate::{
    PageIndex, page::Page, ui::{
        Object, XAlignment, YAlignment, button::{Button, ButtonConfig, regular_button::RegularButton}, config::{
            dimension::{DynamicLength::{self, Full}, ObjectDimension}, position::{ObjectPosition, Position},
        }, container::Container, draw::draw_rectangle_extended, parent::ParentState,
    },
};

pub struct MainMenu {
    player_name: String,
    objects: Vec<Box<dyn Object>>,
}

impl MainMenu {
    pub fn new(player_name: &str) -> MainMenu {
        let create_room_btn =
            RegularButton::new(ObjectPosition::absolute(0.0, 0.0), ButtonConfig::default("Buat Room"))
                .on_click(|| {
                    println!("Terpencet");
                    return None;
                })
                .set_padding(0.0, 50.0)
                .set_dimensions(screen_width() / 2.0, 0.0)
                .set_alignment(Some(XAlignment::Center), None);

        let join_room_btn = RegularButton::new(
            ObjectPosition::absolute(0.0, 150.0),
            ButtonConfig::default("Masuk Room"),
        )
        .on_click(|| {
            println!("Masuk Room");
            return Some(PageIndex::Room as usize);
        })
        .set_padding(0.0, 50.0)
        .set_dimensions(screen_width() / 2.0, 0.0)
        .set_alignment(Some(XAlignment::Center), None);

        let settings_btn = RegularButton::new(
            ObjectPosition::absolute(0.0, 150.0 * 2.0),
            ButtonConfig::default("Pengaturan"),
        )
        .on_click(|| {
            println!("Masuk Room");
            return Some(PageIndex::Room as usize);
        })
        .set_padding(0.0, 50.0)
        .set_dimensions(screen_width() / 2.0, 0.0)
        .set_alignment(Some(XAlignment::Center), None);

        let div = Container::new(
            ObjectPosition::absolute(0.0, 500.0),
            ObjectDimension::dynamic(Full, Full),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(create_room_btn))
        .add_child(Box::new(join_room_btn))
        .add_child(Box::new(settings_btn));

        return MainMenu {
            player_name: String::from(player_name),
            objects: vec![Box::new(div)],
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
        clear_background(BLACK);
        draw_rectangle_extended(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            0.0,
            Color::from_hex(0x7d0202),
            Color::from_hex(0x2b0000),
            30.0,
            0.0,
            BLACK,
        );
        for item in &self.objects {
            item.draw();
        }
    }
}
