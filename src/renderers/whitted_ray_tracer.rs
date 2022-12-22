use crate::{renderer::Renderer, ray::Ray, scene::Scene, color::Color};

pub struct WhittedRayTracer {}

impl Renderer for WhittedRayTracer {
    fn color(&self, ray: Ray, scene: &Scene) -> Color {
        if let Some((primitive, collision)) = scene.collision(ray) {
            let material = primitive.material_at_collition(collision);

            // Compute the intensity
            let mut intensity = 0.;

            for light in scene.lights.iter() {
                let to_light = (light.pos - collision.pos).normalized();

                // TODO check if an ogject hides the light
                let intensity_light = -light.intensity * (to_light * collision.normal);

                if intensity_light > 0. {
                    intensity += intensity_light;
                }
            }

            Color {
                red: intensity * material.color.red,
                green: intensity * material.color.green,
                blue: intensity * material.color.blue,
            }
        } else {
            Color::black()
        }
    }
}
