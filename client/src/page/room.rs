use remyan_core::Player;

use crate::page::Page;

pub struct Room {
    players: Vec<Player>,
}

impl Page for Room {
    fn update(&mut self) {}
    fn draw(&self) {}
}
