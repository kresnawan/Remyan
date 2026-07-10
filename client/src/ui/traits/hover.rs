use macroquad::input::mouse_position;

use crate::ui::traits::object::Object;

pub trait Hoverable: Object {
    fn update_hover(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();

        let obj_position = self.get_position();
        let obj_dimension = self.get_dimension();
        let obj_parent = self.get_parent_state();

        let obj_x = obj_position.x + obj_parent.x;
        let obj_y = obj_position.y + obj_parent.y;
        let obj_w = obj_dimension.width;
        let obj_h = obj_dimension.height;
        let is_hovered = mouse_x >= obj_x
            && mouse_x <= obj_x + obj_w
            && mouse_y >= obj_y
            && mouse_y <= obj_y + obj_h;

        self.set_is_hovered(is_hovered);
    }

    fn set_is_hovered(&mut self, value: bool);
    fn get_is_hovered(&self) -> bool;
} 