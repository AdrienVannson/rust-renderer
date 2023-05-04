use crate::color::Color;
use crate::ray::Ray;
use crate::renderer::Renderer;
use crate::scene::Scene;

/// This basic renderer ignores all the lights in the scene: the color of a ray
/// is simply the color of the object.
pub struct NoLightRenderer {}

impl NoLightRenderer {
    fn color(&self, ray: Ray, scene: &Scene) -> Color {
        if let Some((primitive, collision)) = scene.collision(ray) {
            let material = primitive.material_at_collition(collision);
            material.color
        } else {
            Color::black()
        }
    }
}

impl Renderer for NoLightRenderer {
    fn render(&self, scene: Scene) -> Vec<Vec<(u8, u8, u8)>> {
        let mut image: Vec<Vec<(u8, u8, u8)>> =
            vec![vec![(0, 0, 0); scene.camera.height]; scene.camera.width];

        for x in 0..scene.camera.width {
            for y in 0..scene.camera.height {
                let color = self.color(scene.camera.generate_ray(x, y), &scene);

                image[x][y] = (
                    (255. * color.red) as u8,
                    (255. * color.green) as u8,
                    (255. * color.blue) as u8,
                );
            }
        }

        image
    }
}
