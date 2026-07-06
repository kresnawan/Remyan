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
    objects: Vec<Box<dyn Object>>,
    match_screen_width: bool,
    match_screen_height: bool,
}

impl Container {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        match_screen_width: bool,
        match_screen_height: bool,
    ) -> Container {
        return Container {
            position: ObjectPosition {
                x,
                y,
                x_alignment: None,
                y_alignment: None,
            },
            dimension: ObjectDimension { width, height },
            parent: ParentState {
                x: 0.0,
                y: 0.0,
                height: 0.0,
                width: 0.0,
            },
            objects: Vec::new(),
            match_screen_height,
            match_screen_width,
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
        if self.match_screen_width {
            self.dimension.width = screen_width();
        }

        if self.match_screen_height {
            self.dimension.height = screen_height();
        }

        self.update_alignment(parent_x, parent_y, parent_w, parent_h);

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
