use client::{
    page::{Page, main_menu::MainMenu},
    ui::{
        Object,
        button::{Button, regular_button::RegularButton},
        config::{dimension::Dimension, position::Position},
    },
};
use macroquad::prelude::*;

fn window_config() -> Conf {
    Conf {
        window_title: "Remyan".to_owned(),
        window_width: 1920,
        window_height: 1080,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut app = App::new();
    app.init().await;
}

pub struct App {
    pub current_page: Box<dyn Page>,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_page: Box::new(MainMenu::new("Kresnawan")),
        }
    }

    pub async fn init(&mut self) {
        loop {
            self.current_page.update();
            self.current_page.draw();

            next_frame().await
        }
    }
}
