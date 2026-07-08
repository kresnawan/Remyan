use crate::ui::{
    State, config::position::{ObjectPosition, PositionConfig}, font::Nunito, gradient::Gradient,
};
use macroquad::prelude::*;
pub mod regular_button;

pub trait Button {
    fn new(position: ObjectPosition, config: ButtonConfig) -> Self
    where
        Self: Sized;

    fn on_click<F>(self, callback: F) -> Self
    where
        F: Fn() -> Option<State> + 'static;
}

pub struct ButtonConfig {
    text: String,
    text_size: f32,
    font: Font,
    background_color: Gradient,
    radius: f32,
    text_color: Color,
    outline: f32,
    outline_color: Color,
}

impl ButtonConfig {
    pub fn new(
        text: &str,
        text_size: f32,
        bg_color: Gradient,
        radius: f32,
        outline: f32,
        outline_color: Color,
        text_color: Color,
        font: Font,
    ) -> ButtonConfig {
        ButtonConfig {
            text: String::from(text),
            text_size,
            font,
            radius,
            outline,
            outline_color,
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
            outline: 0.0,
            outline_color: WHITE,
            radius: 10.0,
            text_color: Color::from_hex(0x2b0000),
        }
    }

    pub fn default_with_color(text: &str, color: Gradient) -> ButtonConfig {
        ButtonConfig {
            text: String::from(text),
            background_color: color,
            ..ButtonConfig::default(text)
        }
    }
}

pub struct ButtonAttribute {
    pub outline_thickness: f32,
    pub outline_color: Color,
    pub background_color: Gradient,
    pub corner_radius: f32,
    pub text_color: Color,
    pub on_click_event: Option<Box<dyn Fn() -> Option<State> + 'static>>,
    pub is_clicked: bool,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub shadow_offset: f32,
    pub text_size: f32,
    pub font: Font,
}
