use std::sync::Arc;

use macroquad::{
    color::Color,
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};

use crate::ui::{
    Object,
    config::{
        dimension::{
            DynamicDimension::{Custom, Percent},
            ObjectDimension,
        },
        position::ObjectPosition,
    },
    parent::ParentState,
};

pub struct Container {
    position: ObjectPosition,
    dimension: ObjectDimension,
    parent: ParentState,
    is_flex: bool,
    flex_gap: f32,
    objects: Vec<Box<dyn Object>>,
    background_color: Option<Color>,
}

impl Container {
    pub fn new(
        position: ObjectPosition,
        dimension: ObjectDimension,
        parent: ParentState,
        color: Option<Color>,
    ) -> Container {
        return Container {
            position,
            dimension,
            objects: Vec::new(),
            parent,
            is_flex: false,
            flex_gap: 0.0,
            background_color: color,
        };
    }

    pub fn add_child(mut self, object: Box<dyn Object>) -> Container {
        self.objects.push(object);
        self
    }

    pub fn set_is_flex(mut self, gap: f32) -> Container {
        self.is_flex = true;
        self.flex_gap = gap;

        return self;
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

        if self.is_flex {
            let child_number = self.objects.len();
            let net_width = self.dimension.width - (self.flex_gap * (child_number - 1) as f32);
            let mut counter: f32 = 0.0;
            let child_net_width = 1.0 / child_number as f32 * net_width;

            // let count_width_dimension = move |_, _, _, _| 1.0 / child_number as f32 * net_width;

            for i in &mut self.objects {
                let child_dimension = i.get_dimension();
                let child_position = i.get_position();

                i.set_dimension(ObjectDimension {
                    width: child_net_width,
                    ..child_dimension
                });
                i.set_position(ObjectPosition {
                    x: counter * child_net_width + (self.flex_gap * counter),
                    ..child_position
                });

                counter += 1.0;
            }
        }

        return None;
    }

    fn draw(&self) {
        if let Some(color) = self.background_color {
            draw_rectangle(
                self.position.x + self.parent.x,
                self.position.y + self.parent.y,
                self.dimension.width,
                self.dimension.height,
                color,
            );
        }
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
