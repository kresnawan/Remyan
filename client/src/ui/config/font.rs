use macroquad::text::{Font, load_ttf_font_from_bytes};

pub trait TextFont {
    fn get() -> Font;
}

pub struct Nunito {}

impl Nunito {
    pub fn regular() -> Font {
        let bytes = include_bytes!("../../../assets/fonts/Nunito/Nunito-Regular.ttf");
        let font = load_ttf_font_from_bytes(bytes).unwrap();

        return font;
    }

    pub fn bold() -> Font {
        let bytes = include_bytes!("../../../assets/fonts/Nunito/Nunito-Bold.ttf");
        let font = load_ttf_font_from_bytes(bytes).unwrap();

        return font;
    }

    pub fn black() -> Font {
        let bytes = include_bytes!("../../../assets/fonts/Nunito/Nunito-Black.ttf");
        let font = load_ttf_font_from_bytes(bytes).unwrap();

        return font;
    }
}