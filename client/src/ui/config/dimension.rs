use std::sync::Arc;

#[derive(Clone)]
pub struct ObjectDimension {
    pub width: f32,
    pub height: f32,
    pub width_dyn: Option<DynamicDimension>,
    pub height_dyn: Option<DynamicDimension>
}

impl ObjectDimension {
    pub fn new(width: f32, height: f32, width_dyn: Option<DynamicDimension>, height_dyn: Option<DynamicDimension>) -> Self {
        ObjectDimension { width, height, width_dyn, height_dyn }
    }

    pub fn absolute(width: f32, height: f32) -> Self {
        ObjectDimension { width, height, width_dyn: None, height_dyn: None }
    }

    pub fn dynamic(width_dyn: DynamicDimension, height_dyn: DynamicDimension) -> Self {
        ObjectDimension { width: 0.0, height: 0.0, width_dyn: Some(width_dyn), height_dyn: Some(height_dyn) }
    }
}

#[derive(Clone)]
pub enum DynamicDimension {
    Full,
    Percent(f32),
    // Custom(fn(f32, f32, f32, f32) -> f32)
    Custom(Arc<dyn Fn(f32, f32, f32, f32) -> f32>),
    Flex
}
