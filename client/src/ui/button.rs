use crate::ui::{config::position::PositionConfig, font::Nunito, gradient::Gradient};
use macroquad::prelude::*;
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
    background_color: Gradient,
    radius: f32,
    text_color: Color,
}

impl ButtonConfig {
    pub fn new(
        text: &str,
        text_size: f32,
        bg_color: Gradient,
        radius: f32,
        text_color: Color,
        font: Font,
    ) -> ButtonConfig {
        ButtonConfig {
            text: String::from(text),
            text_size,
            font,
            radius,
            background_color: bg_color,
            text_color: text_color,
        }
    }

    pub fn default(text: &str) -> ButtonConfig {
        ButtonConfig {
            text: String::from(text),
            text_size: 36.0,
            font: Nunito::black(),
            background_color: Gradient::new(
                90.0,
                vec![Color::from_hex(0xfca503), Color::from_hex(0xfc6203)],
            ),
            radius: 10.0,
            text_color: Color::from_hex(0x2b0000),
        }
    }
}
