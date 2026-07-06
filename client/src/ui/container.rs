use macroquad::window::{screen_height, screen_width};

use crate::ui::{
    Object,
    config::{dimension::ObjectDimension, position::ObjectPosition},
    parent::ParentState,
};

pub struct Container {
    position: ObjectPosition,
    dimension: ObjectDimension,
    parent: ParentState,
    objects: Vec<Box<dyn Object>>
}

impl Container {
    pub fn new(
        position: ObjectPosition,
        dimension: ObjectDimension,
        parent: ParentState
    ) -> Container {
        return Container {
            position,
            dimension,
            objects: Vec::new(),
            parent
        };
    }

    pub fn add_child(mut self, object: Box<dyn Object>) -> Container {
        self.objects.push(object);
        self
    }
}

impl Object for Container {
    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
    ) -> Option<usize> {
        self.update_parent_state(parent_x, parent_y, parent_w, parent_h);
        self.update_dimension();
        self.update_alignment();

        for i in &mut self.objects {
            if let Some(n) = i.update(
                Some(self.position.x + self.parent.x),
                Some(self.position.y + self.parent.y),
                Some(self.dimension.width),
                Some(self.dimension.height),
            ) {
                return Some(n);
            }
        }

        return None;
    }

    fn draw(&self) {
        for i in &self.objects {
            i.draw();
        }
    }

    fn get_dimension(&self) -> ObjectDimension {
        return self.dimension.clone();
    }

    fn get_parent_state(&self) -> ParentState {
        return self.parent.clone();
    }

    fn get_position(&self) -> ObjectPosition {
        return self.position.clone();
    }

    fn set_dimension(&mut self, value: ObjectDimension) {
        self.dimension = value;
    }

    fn set_parent_state(&mut self, value: ParentState) {
        self.parent = value;
    }

    fn set_position(&mut self, value: ObjectPosition) {
        self.position = value;
    }
}
