use crate::ui::State;

pub mod main_menu;
pub mod room;

pub trait Page {
    fn update(&mut self, state: &Option<State>) -> Option<State>;
    fn draw(&self) {}
}
