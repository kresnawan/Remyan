use remyan_core::Player;

use crate::{
    PageIndex, page::Page, ui::{
        Object,
        button::{Button, ButtonConfig, regular_button::RegularButton},
        config::position::Position,
    },
};

pub struct Room {
    players: Vec<Player>,
    objects: Vec<Box<dyn Object>>,
}

impl Room {
    pub fn new() -> Self {

        let btn = Box::new(RegularButton::new(
            Position::new(0.0, 0.0),
            ButtonConfig::default("Room"),
        ).on_click(|| {
            return Some(PageIndex::MainMenu as usize)
        }).set_padding(100.0, 50.0));

        Self {
            players: Vec::new(),
            objects: vec![btn],
        }
    }
}

impl Page for Room {
    fn update(&mut self) -> Option<usize> {
        for i in &mut self.objects {
            if let Some(n) = i.update(None, None, None, None) {
                return Some(n);
            }
        }

        return None;
    }
    fn draw(&self) {
        for i in &self.objects {
            i.draw();
        }
    }
}
