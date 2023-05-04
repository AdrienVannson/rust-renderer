use std::{
    sync::{mpsc, Arc},
    thread,
};
use rand::Rng;
use crate::{Color, Ray, Renderer, Scene, Vect};

pub struct MonteCarloRenderer {
    pub iterations_per_pixel: u32,
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

fn one_color(ray: Ray, scene: &Scene) -> Color {
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

fn color(iterations: u32, ray: Ray, scene: &Scene) -> Color {
    //self.one_color(ray, scene)

    let mut sum = (0., 0., 0.);

    for _ in 0..iterations {
        let color = one_color(ray, scene);

        sum.0 += color.red;
        sum.1 += color.green;
        sum.2 += color.blue;
    }

    let f = 1. / iterations as f64;
    Color::new(f * sum.0, f * sum.1, f * sum.2)
}

impl Renderer for MonteCarloRenderer {
    fn render(&self, scene: Scene) -> Vec<Vec<(u8, u8, u8)>> {
        let scene = Arc::new(scene);

        let width = scene.camera.width;
        let height = scene.camera.height;
        let iterations_per_pixel = self.iterations_per_pixel;

        let mut image: Vec<Vec<(u8, u8, u8)>> =
            vec![vec![(0, 0, 0); scene.camera.height]; scene.camera.width];

        // The type of a request is Option<Request>, None ends the thread
        struct Request {
            x: usize,
            y: usize,
        }

        struct Answer {
            sender: usize,
            x: usize,
            y: usize,
            color: Color,
        }

        let (tx_main, rx_main) = mpsc::channel::<Answer>();

        let scene = Arc::new(scene);

        let mut tx_workers = Vec::new();
        let mut handles = Vec::new();

        let workers_count = 4;

        for worker_id in 0..workers_count {
            let scene = Arc::clone(&scene);
            let (tx_worker, rx_worker) = mpsc::channel::<Option<Request>>();
            tx_workers.push(tx_worker);
            let tx_main = tx_main.clone();

            handles.push(thread::spawn(move || {
                while let Some(request) = rx_worker.recv().unwrap() {
                    let (x, y) = (request.x, request.y);

                    // Compute the ray
                    let ray = scene.camera.generate_ray(x, y);

                    // Get the color
                    let color = color(iterations_per_pixel, ray, &scene);

                    tx_main
                        .send(Answer {
                            sender: worker_id,
                            x,
                            y,
                            color,
                        })
                        .unwrap();
                }
            }));
        }

        // Not a real constraint and simplifies a bit the implementation
        assert!(workers_count <= height);

        for x in 0..width {
            for y in 0..height {
                let worker_id = if x == 0 && y < workers_count {
                    y
                } else {
                    let answer = rx_main.recv().unwrap();
                    let (x, y) = (answer.x, answer.y);

                    // Compute the pixel
                    let pixel = (
                        (255. * answer.color.red) as u8,
                        (255. * answer.color.green) as u8,
                        (255. * answer.color.blue) as u8,
                    );

                    image[x][y] = pixel;

                    answer.sender
                };

                tx_workers[worker_id].send(Some(Request { x, y })).unwrap();
            }
        }

        for _ in 0..workers_count {
            let answer = rx_main.recv().unwrap();
            let (x, y) = (answer.x, answer.y);

            // Compute the pixel
            let pixel = (
                (255. * answer.color.red) as u8,
                (255. * answer.color.green) as u8,
                (255. * answer.color.blue) as u8,
            );

            image[x][y] = pixel;
        }

        // End the workers
        for tx in tx_workers {
            tx.send(None).unwrap();
        }

        // End for the treads to finish
        for handle in handles {
            handle.join().unwrap();
        }

        image
    }
}

