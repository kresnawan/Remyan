use macroquad::window::next_frame;

use crate::page::{Page, main_menu::{self, MainMenu}, room::Room};

pub mod ui;
pub mod page;

pub enum PageIndex {
    MainMenu = 0,
    Room = 1
}

pub struct App {
    pub current_page: usize,
    pub pages: Vec<Box<dyn Page>>
}

impl App {
    pub fn new() -> Self {
        let main_menu = Box::new(MainMenu::new("Kresnawan"));
        let room = Box::new(Room::new());
        Self {
            current_page: PageIndex::MainMenu as usize,
            pages: vec![main_menu, room]
        }
    }

    fn get_current_page_mut(&mut self) -> &mut Box<dyn Page> {
        return &mut self.pages[self.current_page]
    }

    fn get_current_page(&mut self) -> &Box<dyn Page> {
        return &self.pages[self.current_page]
    }

    pub async fn init(&mut self) {
        loop {
            if let Some(next_page) = self.get_current_page_mut().update() {
                self.current_page = next_page;
            }
            self.get_current_page().draw();

            next_frame().await
        }
    }
}