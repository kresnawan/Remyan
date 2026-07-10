use macroquad::window::next_frame;

use crate::{page::{Page, Pages, main_menu::MainMenu, room::Room}, state::State};

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
                    next_frame().await;
                    if let Some(next_page) = &self.next_page_to_load {
                        match next_page {
                            Pages::MainMenu => {
                                self.current_page = Some(Box::new(MainMenu::new("Kresna")));
                                self.next_page_to_load = None;
                            }
                            Pages::Room => {
                                self.current_page = Some(Box::new(Room::new()));
                                self.next_page_to_load = None;
                            }
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