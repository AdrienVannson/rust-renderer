use crate::{color::Color, ray::Ray, renderer::Renderer, scene::Scene};

pub struct WhittedRayTracer {}

impl WhittedRayTracer {
    fn color(&self, ray: Ray, scene: &Scene) -> Color {
        if let Some((primitive, collision)) = scene.collision(ray) {
            let material = primitive.material_at_collition(collision);

            // Compute the intensity
            let mut intensity = 0.;

            for light in scene.lights.iter() {
                let to_light = light.pos - collision.pos;

                // Check if an ogject hides the light
                let mut ray_to_light = Ray::new(collision.pos, to_light);
                ray_to_light.move_by(1e-3); // TODO make this value depend on the scene

                if scene.collision_date(ray_to_light) >= to_light.norm() {
                    let intensity_light = light.intensity * (ray_to_light.dir() * collision.normal);

                    if intensity_light > 0. {
                        intensity += intensity_light;
                    }
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

impl Renderer for WhittedRayTracer {
    fn render(&self, scene: &Scene) -> Vec<Vec<(u8, u8, u8)>> {
        let camera = &scene.camera;
        let mut image: Vec<Vec<(u8, u8, u8)>> = vec![vec![(0, 0, 0); camera.height]; camera.width];


        for x in 0..camera.width {
            for y in 0..camera.height {// Compute the pixel
                let color = self.color(camera.generate_ray(x, y), scene);
                
                let pixel = (
                    (255. * color.red) as u8,
                    (255. * color.green) as u8,
                    (255. * color.blue) as u8,
                );

                image[x][y] = pixel;
            }
        };

        image
    }
}
