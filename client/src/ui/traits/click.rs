use macroquad::input::{MouseButton, is_mouse_button_released};

use crate::ui::traits::hover::Hoverable;

pub trait Clickable: Hoverable {
    fn update_is_clicked(&mut self) {
        self.set_is_clicked(self.get_is_hovered() && is_mouse_button_released(MouseButton::Left));
    }

    fn set_is_clicked(&mut self, value: bool);
}