use std::sync::Arc;

use macroquad::text::{Font, load_ttf_font_from_bytes};

pub trait TextFont {
    fn get() -> Font;
}

pub struct Nunito {
    pub regular: Arc<Font>,
    pub bold: Arc<Font>,
    pub black: Arc<Font>
}

impl Nunito {
    pub fn load() -> Self {
        let bytes = include_bytes!("../../../assets/fonts/Nunito/Nunito-Regular.ttf");
        let regular_font = load_ttf_font_from_bytes(bytes).unwrap();

        let bytes = include_bytes!("../../../assets/fonts/Nunito/Nunito-Bold.ttf");
        let bold_font = load_ttf_font_from_bytes(bytes).unwrap();

        let bytes = include_bytes!("../../../assets/fonts/Nunito/Nunito-Black.ttf");
        let black_font = load_ttf_font_from_bytes(bytes).unwrap();

        Self { regular: Arc::new(regular_font), bold: Arc::new(bold_font), black: Arc::new(black_font) }
    }
}