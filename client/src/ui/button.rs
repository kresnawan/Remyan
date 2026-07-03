use macroquad::prelude::*;

use crate::ui::config::{dimension::DimensionConfig, position::PositionConfig};

pub mod regular_button;

pub trait Button {
    fn new<T, U>(position: T, dimension: U, config: ButtonConfig) -> Self
    where
        T: PositionConfig,
        U: DimensionConfig,
        Self: Sized;

    fn on_click<F>(self, callback: F) -> Self
    where
        F: Fn() -> () + 'static;
}

pub struct ButtonConfig {
    text: String,
    text_size: f32
}

impl ButtonConfig {
    pub fn new(text: &str, text_size: f32) -> Self {
        ButtonConfig { text: String::from(text), text_size }
    }
}