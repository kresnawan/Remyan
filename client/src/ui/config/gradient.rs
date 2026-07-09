use macroquad::color::Color;

#[derive(Clone)]
pub struct Gradient {
    pub colors: Vec<Color>,
    pub angle: f32,
}

impl Gradient {
    pub fn new(angle: f32, colors: Vec<Color>) -> Gradient {
        Gradient { colors, angle }
    }

    pub fn primary() -> Gradient {
        Gradient::new(
            90.0,
            vec![Color::from_hex(0xfca503), Color::from_hex(0xfc6203)],
        )
    }

    pub fn secondary() -> Gradient {
        Gradient::new(
            90.0,
            vec![Color::from_hex(0x26b0ff), Color::from_hex(0x083dff)],
        )
    }

    pub fn gray() -> Gradient {
        Gradient::new(
            90.0,
            vec![Color::from_hex(0x7d7d7d1), Color::from_hex(0x5c5c5c)],
        )
    }

    pub fn danger() -> Gradient {
        Gradient::new(
            90.0,
            vec![Color::from_hex(0xe80202), Color::from_hex(0x870000)],
        )
    }
}
