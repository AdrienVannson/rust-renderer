use crate::color::Color;
use crate::ray::Ray;
use crate::renderer::Renderer;
use crate::scene::Scene;

/// This basic renderer ignores all the lights in the scene: the color of a ray
/// is simply the color of the object.
pub struct NoLightRenderer {}

impl Renderer for NoLightRenderer {
    fn color(&self, ray: Ray, scene: &Scene) -> Color {
        if let Some(prim) = scene.collision(ray) {
            let material = prim.material_at_collition(ray);

            material.color
        } else {
            Color::black()
        }
    }
}
