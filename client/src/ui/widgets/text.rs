use std::sync::Arc;

use macroquad::{
    color::{Color, WHITE},
    text::{Font, TextParams, measure_text},
};

use crate::{
    state::State,
    ui::{
        config::{
            dimension::{DynamicDimension, ObjectDimension},
            font::Nunito,
            parent::ParentState,
            position::ObjectPosition,
        },
        traits::object::Object,
    },
    wrapper::draw::draw_text_extended,
};

pub const HEADING_1: f32 = 48.0;
pub const HEADING_2: f32 = 42.0;
pub const HEADING_3: f32 = 36.0;
pub const HEADING_4: f32 = 30.0;
pub const HEADING_5: f32 = 24.0;
pub const HEADING_6: f32 = 18.0;
pub const HEADING_7: f32 = 12.0;
pub const HEADING_8: f32 = 6.0;

#[derive(Clone)]
pub struct TextConfig {
    pub font: Arc<Font>,
    pub color: Color,
    pub font_size: f32,
    pub is_shown: bool,
}

impl TextConfig {
    pub fn default(font: Arc<Nunito>) -> Self {
        TextConfig {
            font: font.regular.clone(),
            color: WHITE,
            font_size: HEADING_3,
            is_shown: true,
        }
    }

    pub fn new(font: Arc<Font>, color: Color, font_size: f32) -> Self {
        TextConfig {
            font,
            color,
            font_size,
            is_shown: true,
        }
    }
}

pub struct Text {
    position: ObjectPosition,
    dimension: ObjectDimension,
    parent: ParentState,
    pub value: String,
    pub config: TextConfig,
}

impl Text {
    pub fn new(value: &str, font: Arc<Nunito>) -> Text {
        let config = TextConfig::default(font);
        let text_dimensions = measure_text(value, Some(&config.font), config.font_size as u16, 1.0);
        Text {
            position: ObjectPosition::absolute(0.0, 0.0),
            dimension: ObjectDimension::absolute(text_dimensions.width, text_dimensions.height),
            parent: ParentState::new(),
            value: String::from(value),
            config,
        }
    }

    pub fn set_position(mut self, value: ObjectPosition) -> Self {
        self.position = value;
        self
    }

    pub fn set_dimension(mut self, value: ObjectDimension) -> Self {
        self.dimension = value;
        self
    }

    pub fn set_config(mut self, value: TextConfig) -> Self {
        self.config = value.clone();
        let new_dimension =
            measure_text(&self.value, Some(&value.font), value.font_size as u16, 1.0);
        self.dimension.width = new_dimension.width;
        self.dimension.height = new_dimension.height;

        self
    }

    pub fn set_font_size(mut self, value: f32) -> Self {
        self.config.font_size = value;
        let new_dimension = measure_text(&self.value, Some(&self.config.font), value as u16, 1.0);
        self.dimension.width = new_dimension.width;
        self.dimension.height = new_dimension.height;

        self
    }

    pub fn wrap_text(mut self) -> Self {
        self.dimension.width_dyn = Some(DynamicDimension::Full);
        self
    }
}

impl Object for Text {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn draw(&self) {
        if self.config.is_shown {
            let current_dimension = measure_text(
                &self.value,
                Some(&self.config.font),
                self.config.font_size as u16,
                1.0,
            );
            draw_text_extended(
                &self.value,
                self.position.x + self.parent.x,
                self.position.y + self.parent.y + current_dimension.offset_y,
                Some(self.dimension.width),
                TextParams {
                    font: Some(&self.config.font),
                    color: self.config.color,
                    font_size: self.config.font_size as u16,
                    ..Default::default()
                },
            );
        }
    }

    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
        _: &Option<State>,
    ) -> Option<State> {
        self.update_parent_state(parent_x, parent_y, parent_w, parent_h);
        self.update_dimension();
        self.update_alignment();

        return None;
    }

    // when Text's width set to dynamic, its wrap width limit will set based on the dynamics
    // if the width_dyn is set to None, the width will depends on the text's length, and it's not going to wrapped
    //
    //
    fn update_dimension(&mut self) {
        let mut current_dimension = self.get_dimension();
        let text_dimension = measure_text(
            &self.value,
            Some(&self.config.font),
            self.config.font_size as u16,
            1.0,
        );
        current_dimension.height = text_dimension.height;
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
                    let res = value(
                        parent_state.x,
                        parent_state.y,
                        parent_state.width,
                        parent_state.height,
                    );
                    current_dimension.width = res;
                }
                DynamicDimension::Grid => {
                    current_dimension.width = parent_state.width;
                }
            }

            self.set_dimension_ref(current_dimension);
        } else {
            self.dimension.width = text_dimension.width;
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

    fn set_dimension_ref(&mut self, value: ObjectDimension) {
        self.dimension = value;
    }

    fn set_parent_state_ref(&mut self, value: ParentState) {
        self.parent = value;
    }

    fn set_position_ref(&mut self, value: ObjectPosition) {
        self.position = value;
    }
}
