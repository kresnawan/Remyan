use crate::{
    state::State,
    ui::{
        config::{dimension::ObjectDimension, position::ObjectPosition},
        widgets::{rectangle::RectangleConfig, text::TextConfig},
    },
};
pub mod regular_button;

pub trait Button {
    fn new(
        position: ObjectPosition,
        dimension: Option<ObjectDimension>,
        text: &str,
        text_config: TextConfig,
        background_config: RectangleConfig,
        shadow_offset: f32
    ) -> Self;

    fn on_click<F>(self, callback: F) -> Self
    where
        F: Fn() -> Option<State> + 'static;
}

pub struct ButtonState {
    pub on_click_event: Option<Box<dyn Fn() -> Option<State> + 'static>>,
    pub is_clicked: bool,
    pub is_hovered: bool,
    pub is_pressed: bool,
}

impl ButtonState {
    pub fn new() -> Self {
        ButtonState {
            on_click_event: None,
            is_clicked: false,
            is_hovered: false,
            is_pressed: false,
        }
    }
}
