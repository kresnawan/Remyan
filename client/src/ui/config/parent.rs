use macroquad::window::{screen_height, screen_width};

#[derive(Clone)]
pub struct ParentState {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
}

impl ParentState {
    pub fn new() -> Self {
        ParentState { x: 0.0, y: 0.0, height: 0.0, width: 0.0 }
    }

    pub fn root() -> ParentState {
        ParentState { x: 0.0, y: 0.0, height: screen_height(), width: screen_width() }
    }
}