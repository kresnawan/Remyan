use macroquad::window::{screen_height, screen_width};

use crate::ui::{
    config::{dimension::ObjectDimension, position::ObjectPosition},
    parent::ParentState,
};

pub mod button;
pub mod config;
pub mod container;
pub mod draw;
pub mod font;
pub mod gradient;
pub mod parent;

pub trait Object {
    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
    ) -> Option<usize>;
    fn draw(&self);

    // Position
    fn get_position(&self) -> ObjectPosition;
    fn set_position(&mut self, value: ObjectPosition);

    // Dimension
    fn get_dimension(&self) -> ObjectDimension;
    fn set_dimension(&mut self, value: ObjectDimension);

    // Parent
    fn get_parent_state(&self) -> ParentState;
    fn set_parent_state(&mut self, value: ParentState);

    fn set_alignment(mut self, x: Option<XAlignment>, y: Option<YAlignment>) -> Self
    where
        Self: Sized,
    {
        let position = self.get_position();
        self.set_position(ObjectPosition {
            x_alignment: x,
            y_alignment: y,
            ..position
        });

        return self;
    }

    fn update_alignment(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
    ) {
        let mut parent_state_temp = self.get_parent_state();
        let mut position_temp = self.get_position();
        let dimension_temp = self.get_dimension();

        if let Some(value) = parent_x {
            parent_state_temp.x = value;
        }

        if let Some(value) = parent_y {
            parent_state_temp.y = value;
        }

        if let Some(value) = parent_w {
            parent_state_temp.width = value;
        } else {
            parent_state_temp.width = screen_width();
        }

        if let Some(value) = parent_h {
            parent_state_temp.height = value;
        } else {
            parent_state_temp.height = screen_height();
        }

        if let Some(value) = &position_temp.x_alignment {
            match value {
                XAlignment::Left => {
                    position_temp.x = parent_state_temp.x;
                }
                XAlignment::Center => {
                    position_temp.x = parent_state_temp.width / 2.0 - dimension_temp.width / 2.0;
                }
                XAlignment::Right => {
                    position_temp.x = parent_state_temp.width - dimension_temp.width;
                }
            }
        }

        if let Some(value) = &position_temp.y_alignment {
            match value {
                YAlignment::Top => {
                    position_temp.y = parent_state_temp.y;
                }
                YAlignment::Center => {
                    position_temp.y = parent_state_temp.height / 2.0 - dimension_temp.height / 2.0;
                }
                YAlignment::Bottom => {
                    position_temp.y = parent_state_temp.height - dimension_temp.height;
                }
            }
        }

        self.set_parent_state(parent_state_temp);
        self.set_position(position_temp);
    }
}

#[derive(Clone)]
pub enum XAlignment {
    Left,
    Center,
    Right,
}

#[derive(Clone)]
pub enum YAlignment {
    Top,
    Center,
    Bottom,
}
