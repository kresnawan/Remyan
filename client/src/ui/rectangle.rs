use macroquad::color::Color;

use crate::ui::{
    Object, State, config::{dimension::ObjectDimension, position::ObjectPosition}, draw::draw_rectangle_extended, gradient::Gradient, parent::ParentState,
};

pub struct RectangleConfig {
    pub corner_radius: f32,
    pub color: Gradient,
    pub outline: f32,
    pub outline_color: Color,
}

impl RectangleConfig {
    pub fn new(corner_radius: f32, color: Gradient, outline: f32, outline_color: Color) -> Self {
        RectangleConfig { corner_radius, color, outline, outline_color }
    }
}

pub struct Rectangle {
    pub position: ObjectPosition,
    pub dimension: ObjectDimension,
    pub parent: ParentState,
    pub config: RectangleConfig,
}

impl Rectangle {
    pub fn new(
        position: ObjectPosition,
        dimension: ObjectDimension,
        parent: ParentState,
        config: RectangleConfig,
    ) -> Self {
        let second_color = if config.color.colors.len() < 2 {
            config.color.colors[0]
        } else {
            config.color.colors[1]
        };

        let config = RectangleConfig {
            color: Gradient {
                colors: vec![config.color.colors[0], second_color],
                ..config.color
            },
            ..config
        };

        Self {
            position,
            dimension,
            parent,
            config,
        }
    }
}

impl Object for Rectangle {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
        _: &Option<State>
    ) -> Option<State> {
        self.update_parent_state(parent_x, parent_y, parent_w, parent_h);
        self.update_dimension();
        self.update_alignment();

        return None;
    }

    fn draw(&self) {
        draw_rectangle_extended(
            self.position.x + self.parent.x,
            self.position.y + self.parent.y,
            self.dimension.width,
            self.dimension.height,
            self.config.corner_radius,
            self.config.color.colors[0],
            self.config.color.colors[1],
            self.config.color.angle,
            self.config.outline,
            self.config.outline_color,
        );
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

    fn set_position_ref(&mut self, value: ObjectPosition) {
        self.position = value;
    }

    fn set_parent_state_ref(&mut self, value: ParentState) {
        self.parent = value;
    }
}
