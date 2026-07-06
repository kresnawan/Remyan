#[derive(Clone)]
pub struct ObjectDimension {
    pub width: f32,
    pub height: f32,
    pub width_dyn: Option<DynamicLength>,
    pub height_dyn: Option<DynamicLength>
}

impl ObjectDimension {
    pub fn new(width: f32, height: f32, width_dyn: Option<DynamicLength>, height_dyn: Option<DynamicLength>) -> Self {
        ObjectDimension { width, height, width_dyn, height_dyn }
    }

    pub fn absolute(width: f32, height: f32) -> Self {
        ObjectDimension { width, height, width_dyn: None, height_dyn: None }
    }

    pub fn dynamic(width_dyn: DynamicLength, height_dyn: DynamicLength) -> Self {
        ObjectDimension { width: 0.0, height: 0.0, width_dyn: Some(width_dyn), height_dyn: Some(height_dyn) }
    }
}

#[derive(Clone)]
pub enum DynamicLength {
    Full,
    Percent(f32)
}
