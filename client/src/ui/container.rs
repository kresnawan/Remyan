use macroquad::window::{screen_height, screen_width};

use crate::ui::Object;

pub struct Container {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    parent_x: f32,
    parent_y: f32,
    match_screen_width: bool,
    match_screen_height: bool,
    objects: Vec<Box<dyn Object>>,
}

impl Container {
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        match_screen_width: bool,
        match_screen_height: bool,
    ) -> Container {
        return Self {
            x,
            y,
            width: w,
            height: h,
            parent_x: 0.0,
            parent_y: 0.0,
            match_screen_width,
            match_screen_height,
            objects: Vec::new(),
        };
    }

    pub fn add_child(mut self, object: Box<dyn Object>) -> Container {
        self.objects.push(object);
        self
    }

}

impl Object for Container {
    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        _: Option<f32>,
        _: Option<f32>,
    ) -> Option<usize> {
        if self.match_screen_width {
            self.width = screen_width();
        }

        if self.match_screen_height {
            self.height = screen_height();
        }

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
                Some(self.width),
                Some(self.height),
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
