pub mod button;
pub mod config;
pub mod container;
pub mod draw;
pub mod font;
pub mod gradient;

pub trait Object {
    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
    ) -> Option<usize>;
    fn draw(&self) {}
}

pub enum XAlignment {
    Left,
    Center,
    Right,
}

pub enum YAlignment {
    Top,
    Center,
    Bottom,
}
