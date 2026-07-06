pub trait PositionConfig {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
}

pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position { x, y }
    }
}

impl PositionConfig for Position {
    fn get_x(&self) -> f32 {
        return self.x;
    }

    fn get_y(&self) -> f32 {
        return self.y;
    }
}

use crate::ui::{XAlignment, YAlignment};

#[derive(Clone)]
pub struct ObjectPosition {
    pub x: f32,
    pub y: f32,
    pub x_alignment: Option<XAlignment>,
    pub y_alignment: Option<YAlignment>,
}
