use crate::Color;

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color,
}

impl Material {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
