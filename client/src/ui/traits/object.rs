use std::any::Any;

use macroquad::window::{screen_height, screen_width};

use crate::{state::State, ui::config::{dimension::{DynamicDimension, ObjectDimension}, parent::ParentState, position::{DynamicPosition, ObjectPosition}}};

pub trait Object {
    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
        state: &Option<State>
    ) -> Option<State>;
    fn draw(&self);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    // Position
    fn get_position(&self) -> ObjectPosition;
    fn set_position_ref(&mut self, value: ObjectPosition);

    // Dimension
    fn get_dimension(&self) -> ObjectDimension;
    fn set_dimension_ref(&mut self, value: ObjectDimension);

    // Parent
    fn get_parent_state(&self) -> ParentState;
    fn set_parent_state_ref(&mut self, value: ParentState);

    fn update_dimension(&mut self) {
        let mut current_dimension = self.get_dimension();
        let parent_state = self.get_parent_state();

        if let Some(n) = &current_dimension.width_dyn {
            match n {
                DynamicDimension::Full => {
                    current_dimension.width = parent_state.width;
                }
                DynamicDimension::Percent(value) => {
                    current_dimension.width = (value / 100.0) * parent_state.width;
                }
                DynamicDimension::Custom(value) => {
                    let res = value(parent_state.x, parent_state.y, parent_state.width, parent_state.height);
                    current_dimension.width = res;
                }
                DynamicDimension::Grid => {}
            }
        }

        if let Some(n) = &current_dimension.height_dyn {
            match n {
                DynamicDimension::Full => {
                    current_dimension.height = parent_state.height;
                }
                DynamicDimension::Percent(value) => {
                    current_dimension.height = (value / 100.0) * parent_state.height;
                }
                DynamicDimension::Custom(value) => {
                    let res = value(parent_state.x, parent_state.y, parent_state.width, parent_state.height);
                    current_dimension.height = res;
                }
                DynamicDimension::Grid => {}
            }
        }

        self.set_dimension_ref(current_dimension);
    }

    fn set_alignment(mut self, x: Option<DynamicPosition>, y: Option<DynamicPosition>) -> Self
    where
        Self: Sized,
    {
        let position = self.get_position();
        self.set_position_ref(ObjectPosition {
            x_dyn: x,
            y_dyn: y,
            ..position
        });

        return self;
    }

    fn update_parent_state(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
    ) {
        let mut parent_state_temp = self.get_parent_state();

        if let Some(value) = parent_x {
            parent_state_temp.x = value;
        } else {
            parent_state_temp.x = 0.0;
        }

        if let Some(value) = parent_y {
            parent_state_temp.y = value;
        } else {
            parent_state_temp.y = 0.0;
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

        self.set_parent_state_ref(parent_state_temp);
    }

    fn update_alignment(&mut self) {
        let parent_state_temp = self.get_parent_state();
        let mut position_temp = self.get_position();
        let dimension_temp = self.get_dimension();

        if let Some(value) = &position_temp.x_dyn {
            match value {
                DynamicPosition::Start => {
                    position_temp.x = 0.0;
                }
                DynamicPosition::Center => {
                    position_temp.x = parent_state_temp.width / 2.0 - dimension_temp.width / 2.0;
                }
                DynamicPosition::End => {
                    position_temp.x = parent_state_temp.width - dimension_temp.width;
                }
                DynamicPosition::Custom(value) => {
                    position_temp.x = value(parent_state_temp.x, parent_state_temp.y, parent_state_temp.width, parent_state_temp.height);
                }
                DynamicPosition::Flex => {}
                DynamicPosition::Grid => {}
            }
        }

        if let Some(value) = &position_temp.y_dyn {
            match value {
                DynamicPosition::Start => {
                    position_temp.y = 0.0;
                }
                DynamicPosition::Center => {
                    position_temp.y = parent_state_temp.height / 2.0 - dimension_temp.height / 2.0;
                }
                DynamicPosition::End => {
                    position_temp.y = parent_state_temp.height - dimension_temp.height;
                }
                DynamicPosition::Custom(value) => {
                    position_temp.y = value(parent_state_temp.x, parent_state_temp.y, parent_state_temp.width, parent_state_temp.height);
                }
                DynamicPosition::Flex => {}
                DynamicPosition::Grid => {}
            }
        }

        self.set_position_ref(position_temp);
    }
}
