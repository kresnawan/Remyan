pub mod button;
pub mod config;
pub mod font;
pub mod draw;
pub mod gradient;

pub trait Object {
    fn update(&mut self) -> Option<usize>;
    fn draw(&self) {}
}