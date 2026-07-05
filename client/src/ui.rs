use remyan_core::App;

use crate::ui::config::{dimension::DimensionConfig, position::PositionConfig};

pub mod button;
pub mod config;
pub mod font;

pub trait Object {
    fn update(&mut self) -> Option<usize>;
    fn draw(&self) {}
}