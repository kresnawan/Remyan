use crate::ui::config::{dimension::DimensionConfig, position::PositionConfig};

pub mod button;
pub mod config;

pub trait Object {
    fn update(&mut self) {}
    fn draw(&self) {}
}