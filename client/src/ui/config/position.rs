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

impl ObjectPosition {
    pub fn new(x: f32, y: f32, x_alignment: Option<XAlignment>, y_alignment: Option<YAlignment>) -> ObjectPosition {
        ObjectPosition { x, y, x_alignment, y_alignment }
    }
    
    pub fn absolute(x: f32, y: f32) -> Self {
        ObjectPosition { x, y, x_alignment: None, y_alignment: None }
    }

    pub fn dynamic(x_alignment: XAlignment, y_alignment: YAlignment) -> Self {
        ObjectPosition { x: 0.0, y: 0.0, x_alignment: Some(x_alignment), y_alignment: Some(y_alignment) }
    }
}
