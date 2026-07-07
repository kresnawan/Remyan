use std::sync::Arc;

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

#[derive(Clone)]
pub enum DynamicPosition {
    Start,
    Center,
    End,
    Custom(Arc<dyn Fn(f32, f32, f32, f32) -> f32>)
}

#[derive(Clone)]
pub struct ObjectPosition {
    pub x: f32,
    pub y: f32,
    pub x_dyn: Option<DynamicPosition>,
    pub y_dyn: Option<DynamicPosition>,
}

impl ObjectPosition {
    pub fn new(x: f32, y: f32, x_dyn: Option<DynamicPosition>, y_dyn: Option<DynamicPosition>) -> ObjectPosition {
        ObjectPosition { x, y, x_dyn, y_dyn }
    }
    
    pub fn absolute(x: f32, y: f32) -> Self {
        ObjectPosition { x, y, x_dyn: None, y_dyn: None }
    }

    pub fn dynamic(x_alignment: DynamicPosition, y_alignment: DynamicPosition) -> Self {
        ObjectPosition { x: 0.0, y: 0.0, x_dyn: Some(x_alignment), y_dyn: Some(y_alignment) }
    }
}
