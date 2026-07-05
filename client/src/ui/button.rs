use macroquad::prelude::*;
use crate::ui::{config::{dimension::DimensionConfig, position::PositionConfig}, font::Nunito};
pub mod regular_button;

pub trait Button {
    fn new<T>(position: T, config: ButtonConfig) -> Self
    where
        T: PositionConfig,
        Self: Sized;

    fn on_click<F>(self, callback: F) -> Self
    where
        F: Fn() -> Option<usize> + 'static;
}

pub struct ButtonConfig {
    text: String,
    text_size: f32,
    font: Font,
    background_color: Color,
    text_color: Color,
}

impl ButtonConfig {
    pub fn new(text: &str, text_size: f32, bg_color: Color, text_color: Color, font: Font) -> ButtonConfig {
        ButtonConfig {
            text: String::from(text),
            text_size,
            font,
            background_color: bg_color,
            text_color: text_color,
        }
    }

    pub fn default(text: &str) -> ButtonConfig {
        ButtonConfig {
            text: String::from(text),
            text_size: 24.0,
            font: Nunito::regular(),
            background_color: Color::new(0.07, 0.45, 0.80, 1.0),
            text_color: Color::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}
