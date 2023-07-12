use crate::Color;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub color: Color,
}

impl Material {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
