pub trait DimensionConfig {
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn get_text_size(&self) -> f32;
}

pub struct Dimension {
    width: f32,
    height: f32,
}

pub struct TextDimension {
    size: f32
}

impl Dimension {
    pub fn new(width: f32, height: f32) -> Dimension {
        Dimension { width, height }
    }
}

impl DimensionConfig for Dimension {
    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn get_text_size(&self) -> f32 {
        0.0
    }
}
