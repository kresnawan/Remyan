use macroquad::color::Color;

pub struct Gradient {
    pub colors: Vec<Color>,
    pub angle: f32
}

impl Gradient {
    pub fn new(angle: f32, colors:Vec<Color>) -> Gradient {
        Gradient { colors, angle }
    }
}