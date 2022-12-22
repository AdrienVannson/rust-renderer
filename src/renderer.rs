use crate::color::Color;
use crate::ray::Ray;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }

    pub fn color(&self, ray: Ray, remaining_depth: i32) -> Color {
        if remaining_depth == 0 {
            return Color::black();
        }

        Color::black()
    }
}
