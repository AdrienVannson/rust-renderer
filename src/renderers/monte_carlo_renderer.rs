use crate::{Color, Image, Ray, Renderer, Scene, Vect};
use rand::{thread_rng, Rng};
use std::{
    f64::consts::PI,
    sync::{mpsc, Arc},
    thread,
};

#[derive(Copy, Clone, Debug)]
pub enum SamplingMethod {
    IndependantSamples,
    RegularGrid,
}

pub struct MonteCarloRenderer {
    pub iterations_per_pixel: u32,
    pub sampling_method: SamplingMethod,
}

// u1 and u2 must be random variables uniformly generated in [0, 1].
fn random_vector_in_half_space(dir: Vect, u1: f64, u2: f64) -> Vect {
    let theta = 2.0 * PI * u1;
    let phi = (2.0 * u2 - 1.0).acos();

    let x = phi.sin() * theta.cos();
    let y = phi.sin() * theta.sin();
    let z = phi.cos();

    let vect = Vect::new(x, y, z);

    if vect * dir < 0. {
        -vect
    } else {
        vect
    }
}

fn generate_samples_regular_grid(samples_count: u32) -> Vec<(f64, f64)> {
    let root = (samples_count as f64).sqrt() as u32;
    assert_eq!(root * root, samples_count);

    let mut samples = Vec::new();

    for i in 0..root {
        for j in 0..root {
            samples.push((
                // Not 0.5 to prevent rays from being parallel to the walls
                (i as f64 + 0.505) / root as f64,
                ((j as f64 + 0.505) / root as f64),
            ));
        }
    }

    samples
}

fn generate_samples_uniform_jitter(samples_count: u32) -> Vec<(f64, f64)> {
    let root = (samples_count as f64).sqrt() as u32;
    assert_eq!(root * root, samples_count);

    let mut samples = Vec::new();

    let jitter_x = thread_rng().gen::<f64>() - 0.5;
    let jitter_y = thread_rng().gen::<f64>() - 0.5;

    for i in 0..root {
        for j in 0..root {
            samples.push((
                (i as f64 + 0.5 + jitter_x) / root as f64,
                (j as f64 + 0.5 + jitter_y) / root as f64,
            ));
        }
    }

    samples
}

fn generate_independant_samples(samples_count: u32) -> Vec<(f64, f64)> {
    let mut samples = Vec::new();

    for _ in 0..samples_count {
        samples.push((
            thread_rng().gen::<f64>(),
            thread_rng().gen::<f64>()
        ));
    }

    samples
}

fn one_color(ray: Ray, scene: &Scene, sample: (f64, f64)) -> Color {
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

        let next_dir = random_vector_in_half_space(collision.normal, sample.0, sample.1);
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
                let intensity = 3. * next_dir * (collision.normal);
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

fn color(ray: Ray, scene: &Scene, samples: &Vec<(f64, f64)>) -> Color {
    let mut sum = (0., 0., 0.);

    for sample in samples {
        let color = one_color(ray, scene, *sample);

        sum.0 += color.red;
        sum.1 += color.green;
        sum.2 += color.blue;
    }

    let f = 1. / samples.len() as f64;
    Color::new(f * sum.0, f * sum.1, f * sum.2)
}

impl Renderer for MonteCarloRenderer {
    fn render(&self, scene: Scene) {
        let scene = Arc::new(scene);

        let width = scene.camera.width;
        let height = scene.camera.height;
        let iterations_per_pixel = self.iterations_per_pixel;

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

        let workers_count = 32;

        for worker_id in 0..workers_count {
            let scene = Arc::clone(&scene);
            let (tx_worker, rx_worker) = mpsc::channel::<Option<Request>>();
            tx_workers.push(tx_worker);
            let tx_main = tx_main.clone();

            let sampling_method = self.sampling_method;

            handles.push(thread::spawn(move || {
                while let Some(request) = rx_worker.recv().unwrap() {
                    let (x, y) = (request.x, request.y);

                    // Compute the ray
                    let ray = scene.camera.generate_ray(x, y);

                    // Get the color
                    let color = color(
                        ray,
                        &scene,
                        &/*match sampling_method {
                            SamplingMethod::IndependantSamples => {
                                generate_samples_uniform_jitter(iterations_per_pixel)
                            }
                            SamplingMethod::RegularGrid => {
                                generate_samples_regular_grid(iterations_per_pixel)
                            }
                        },*/
                        generate_independant_samples(iterations_per_pixel)
                    );

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

        let mut image = Image::new(scene.camera.width, scene.camera.height);

        //for it in 0..1000 {
        for it in 0..1000000 {
            println!("Iteration {}...", it);

            for x in 0..width {
                for y in 0..height {
                    let worker_id = if x == 0 && y < workers_count {
                        y
                    } else {
                        let answer = rx_main.recv().unwrap();
                        let (x, y) = (answer.x, answer.y);

                        let prev_color = image.pixel(x, y);

                        let it = it as f64;
                        image.set_pixel(
                            x,
                            y,
                            Color::new(
                                (it * prev_color.red + answer.color.red) / (it + 1.),
                                (it * prev_color.green + answer.color.green) / (it + 1.),
                                (it * prev_color.blue + answer.color.blue) / (it + 1.),
                            ),
                        );

                        answer.sender
                    };

                    tx_workers[worker_id].send(Some(Request { x, y })).unwrap();
                }
            }

            for _ in 0..workers_count {
                let answer = rx_main.recv().unwrap();
                let (x, y) = (answer.x, answer.y);

                let prev_color = image.pixel(x, y);

                let it = it as f64;
                image.set_pixel(
                    x,
                    y,
                    Color::new(
                        (it * prev_color.red + answer.color.red) / (it + 1.),
                        (it * prev_color.green + answer.color.green) / (it + 1.),
                        (it * prev_color.blue + answer.color.blue) / (it + 1.),
                    ),
                );
            }

            image.export(&format!(
                "output-{:0>4}.png",
                self.iterations_per_pixel * (it + 1)
            ));
            image.raw_export(&format!(
                "raw-output-{:0>4}",
                self.iterations_per_pixel * (it + 1)
            ));
        }

        // End the workers
        for tx in tx_workers {
            tx.send(None).unwrap();
        }

        // End for the treads to finish
        for handle in handles {
            handle.join().unwrap();
        }

        image.export("output.png");
    }
}
