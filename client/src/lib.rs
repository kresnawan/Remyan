use macroquad::{
    color::BLACK,
    window::{clear_background, next_frame},
};

use crate::{
    page::{Page, main_menu::MainMenu, room::Room},
    ui::State,
};

pub mod page;
pub mod ui;

#[derive(Clone)]
pub enum Pages {
    MainMenu,
    Room,
}

pub enum GameState {
    Loading,
    Running,
    Uninitialized,
}

pub struct App {
    pub current_page: Option<Box<dyn Page>>,
    pub game_state: GameState,
    pub pre_allocated_pages: Vec<Box<dyn Page>>,
    pub next_page_to_load: Option<Pages>,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_page: None,
            game_state: GameState::Uninitialized,
            pre_allocated_pages: Vec::new(),
            next_page_to_load: Some(Pages::MainMenu),
        }
    }

    pub async fn init(&mut self) {
        let mut global_state: Option<State> = None;
        self.game_state = GameState::Loading;
        loop {
            match self.game_state {
                GameState::Loading => {
                    clear_background(BLACK);
                    if let Some(next_page) = &self.next_page_to_load {
                        match next_page {
                            Pages::MainMenu => {
                                self.current_page = Some(Box::new(MainMenu::new("Kresna")))
                            }
                            Pages::Room => self.current_page = Some(Box::new(Room::new())),
                        }

                        self.game_state = GameState::Running;
                    }
                }

                GameState::Running => {
                    if let Some(state) = self.current_page.as_mut().unwrap().update(&global_state) {
                        match state {
                            State::MovePage(next_page) => {
                                self.game_state = GameState::Loading;
                                self.next_page_to_load = Some(next_page);
                                global_state = None;
                            }
                            _ => global_state = Some(state),
                        }
                    }
                    self.current_page.as_ref().unwrap().draw();
                }

                _ => {}
            }

            next_frame().await
        }
    }
}
