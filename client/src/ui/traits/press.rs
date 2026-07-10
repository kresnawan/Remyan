use macroquad::input::{MouseButton, is_mouse_button_down};

use crate::ui::traits::hover::Hoverable;

pub trait Pressable: Hoverable {
    fn update_is_pressed(&mut self) {
        self.set_is_pressed(self.get_is_hovered() && is_mouse_button_down(MouseButton::Left));
    }

    fn set_is_pressed(&mut self, value: bool);
}