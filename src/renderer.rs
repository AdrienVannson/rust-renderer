use crate::color::Color;
use crate::ray::Ray;
use crate::scene::Scene;

pub trait Renderer: Send + Sync {
    fn color(&self, ray: Ray, scene: &Scene) -> Color;
}
