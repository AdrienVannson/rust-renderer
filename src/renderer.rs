use crate::color::Color;
use crate::ray::Ray;
use crate::scene::Scene;

pub trait Renderer {
    fn color(&self, ray: Ray, scene: &Scene) -> Color;
}
