use client::ui::button::{Button, Dimension, Object, Position, RegularButton};
use macroquad::prelude::*;
use remyan_core::Player;

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
        let mut btn = RegularButton::new(Position::new(100.0, 100.0), Dimension::new(100.0, 50.0));
        btn.on_click(|| println!("Clicked"));
        Self {
            current_page: Box::new(MainMenu::new(
                String::from("Kresnawan"),
                vec![Box::from(btn)],
            )),
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

pub struct MainMenu {
    player_name: String,
    objects: Vec<Box<dyn Object>>,
}

impl MainMenu {
    pub fn new(player_name: String, objects: Vec<Box<dyn Object>>) -> Self {
        Self {
            player_name,
            objects,
        }
    }
}

pub struct Room {
    players: Vec<Player>,
}

impl Page for Room {
    fn update(&mut self) {}
    fn draw(&self) {}
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

pub trait Page {
    fn update(&mut self) {
        
    }
    fn draw(&self) {}
}
