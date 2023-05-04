use crate::{Color, Ray, Renderer, Scene, Vect};
use rand::Rng;
/*
pub struct MonteCarloRenderer {
    pub iterations_per_pixel: i32,
}

fn random_vector_in_half_space(dir: Vect) -> Vect {
    let mut rng = rand::thread_rng();

    let mut vect = Vect::new(
        2. * rng.gen::<f64>() - 1.,
        2. * rng.gen::<f64>() - 1.,
        2. * rng.gen::<f64>() - 1.,
    );

    if vect.squared_norm() > 1. {
        return random_vector_in_half_space(dir);
    }

    vect.normalize();

    if vect * dir < 0. {
        vect = -vect;
    }

    vect
}

impl MonteCarloRenderer {
    fn one_color(&self, ray: Ray, scene: &Scene) -> Color {
        if let Some((primitive, collision)) = scene.collision(ray) {
            let material = primitive.material_at_collition(collision);

            let color = primitive.material_at_collition(collision).color;

            // We hit the light
            if color.red == 1. && color.blue == 1. && color.green == 0. {
                return Color::white();
            }

            // Compute the intensity
            /*let mut intensity = 0.;

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
            }*/

            let next_dir = random_vector_in_half_space(collision.normal);
            let mut next_ray = Ray {
                pos: collision.pos,
                dir: next_dir,
            };
            next_ray.move_by(1e-3);

            if let Some((next_primitive, next_col)) = scene.collision(next_ray) {
                let color = next_primitive.material_at_collition(next_col).color;

                // We hit the light
                if color.red == 1. && color.blue == 1. && color.green == 0. {
                    // Use the intensity from the light
                    let intensity = 12. * next_dir * (collision.normal);
                    // TODO: pourquoi ne fonctionne pas avec (next_col.pos - collision.pos).normalized() ?

                    assert!(intensity >= 0.);

                    return Color {
                        red: intensity * material.color.red,
                        green: intensity * material.color.green,
                        blue: intensity * material.color.blue,
                    };
                }
            }

            Color::black()
        } else {
            Color::black()
        }
    }
}

impl Renderer for MonteCarloRenderer {
    fn color(&self, ray: Ray, scene: &Scene) -> Color {
        //self.one_color(ray, scene)

        let mut sum = (0., 0., 0.);

        for _ in 0..self.iterations_per_pixel {
            let color = self.one_color(ray, scene);

            sum.0 += color.red;
            sum.1 += color.green;
            sum.2 += color.blue;
        }

        let f = 1. / self.iterations_per_pixel as f64;
        Color::new(f * sum.0, f * sum.1, f * sum.2)
    }
}
*/