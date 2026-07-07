use macroquad::color::Color;

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
            vec![Color::from_hex(0xd1d1d1), Color::from_hex(0x8a8a8a)],
        )
    }

    pub fn danger() -> Gradient {
        Gradient::new(
            90.0,
            vec![Color::from_hex(0xe80202), Color::from_hex(0x870000)],
        )
    }
}
