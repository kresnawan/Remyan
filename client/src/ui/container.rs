use macroquad::{
    input::KeyCode::F,
    window::{screen_height, screen_width},
};

use crate::ui::Object;

pub struct Container<F, G> {
    x: f32,
    y: f32,
    width: F,
    height: G,
    parent_x: f32,
    parent_y: f32,
    objects: Vec<Box<dyn Object>>,
}

impl<F, G> Container<F, G>
where
    F: Fn() -> f32,
    G: Fn() -> f32,
{
    pub fn new(x: f32, y: f32, width: F, height: G) -> Container<F, G> {
        return Self {
            x,
            y,
            width: width,
            height: height,
            parent_x: 0.0,
            parent_y: 0.0,
            objects: Vec::new(),
        };
    }

    pub fn add_child(mut self, object: Box<dyn Object>) -> Container<F, G> {
        self.objects.push(object);
        self
    }
}

impl<F, G> Object for Container<F, G>
where
    F: Fn() -> f32,
    G: Fn() -> f32
{
    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        _: Option<f32>,
        _: Option<f32>,
    ) -> Option<usize> {
        let width_attr = &self.width;
        let height_attr = &self.height;

        let width = width_attr();
        let height = height_attr();

        if let Some(value) = parent_x {
            self.parent_x = value;
        }
        if let Some(value) = parent_y {
            self.parent_y = value;
        }
        for i in &mut self.objects {
            if let Some(n) = i.update(
                Some(self.x + self.parent_x),
                Some(self.y + self.parent_y),
                Some(width),
                Some(height),
            ) {
                return Some(n);
            }
        }

        return None;
    }

    fn draw(&self) {
        for i in &self.objects {
            i.draw();
        }
    }
}
