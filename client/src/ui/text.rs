use macroquad::{
    color::{Color, WHITE},
    text::{Font, TextParams, draw_text_ex, measure_text},
};

use crate::ui::{
    HEADING_2, HEADING_3, HEADING_4, Object, State, config::{dimension::{DynamicDimension, ObjectDimension}, position::ObjectPosition}, draw::draw_text_extended, font::Nunito, parent::ParentState,
};

pub struct TextConfig {
    font: Font,
    color: Color,
    font_size: f32,
}

impl TextConfig {
    pub fn default() -> Self {
        TextConfig {
            font: Nunito::regular(),
            color: WHITE,
            font_size: HEADING_3,
        }
    }
}

pub struct Text {
    position: ObjectPosition,
    dimension: ObjectDimension,
    parent: ParentState,
    value: String,
    config: TextConfig,
}

impl Text {
    pub fn new(value: &str) -> Text {
        let config = TextConfig::default();
        let text_dimensions = measure_text(value, Some(&config.font), config.font_size as u16, 1.0);
        Text {
            position: ObjectPosition::absolute(0.0, 0.0),
            dimension: ObjectDimension::absolute(text_dimensions.width, text_dimensions.height),
            parent: ParentState::new(),
            value: String::from(value),
            config: TextConfig::default(),
        }
    }

    pub fn set_position(mut self, value: ObjectPosition) -> Self {
        self.position = value;
        self
    }

    pub fn set_config(mut self, value: TextConfig) -> Self {
        self.config = value;
        self
    }

    pub fn set_font_size(mut self, value: f32) -> Self {
        self.config.font_size = value;
        let new_dimension = measure_text(&self.value, Some(&self.config.font), value as u16, 1.0);
        self.dimension.width = new_dimension.width;
        
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
        draw_text_extended(
            &self.value,
            self.position.x + self.parent.x,
            self.position.y + self.parent.y + self.dimension.height,
            Some(self.dimension.width),
            TextParams {
                font: Some(&self.config.font),
                color: self.config.color,
                font_size: self.config.font_size as u16,
                ..Default::default()
            },
        );
    }

    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
        state: &Option<State>
    ) -> Option<State> {
        self.update_parent_state(parent_x, parent_y, parent_w, parent_h);
        self.update_dimension();
        self.update_alignment();

        return None;
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
