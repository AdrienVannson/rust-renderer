use crate::sampler::Sampler;
use crate::warping::to_cosine_directed_hemisphere;
use crate::{Color, Image, Ray, Renderer, Scene};
use derive_builder::Builder;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use std::{
    sync::{mpsc, Arc},
    thread,
};

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct MonteCarloRenderer<S: Sampler> {
    steps_count: u32,
    iterations_per_step_count: u32,
    output_folder: String,

    #[builder(default = "Color::black()")]
    ambient_occlusion: Color,

    // Given a unique ID representing the thread, returns a new sampler
    sampler_factory: fn(usize) -> S,
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

fn one_color(ray: Ray, scene: &Scene, sample: [f64; 2], ambient_occlusion: Color) -> Color {
    if let Some((primitive, collision)) = scene.collision(ray) {
        let material = primitive.material_at_collision(collision);

        let color = primitive.material_at_collision(collision).color;

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

        // Importance sampling
        let next_dir = to_cosine_directed_hemisphere(collision.normal, sample);
        let mut next_ray = Ray {
            pos: collision.pos,
            dir: next_dir,
        };
        next_ray.move_by(1e-3);

        if let Some((next_primitive, next_col)) = scene.collision(next_ray) {
            let color = next_primitive.material_at_collision(next_col).color;

            // We hit the light
            if color.red == 1. && color.blue == 1. && color.green == 0. {
                // TODO Use the intensity from the light
                // No need to add a cosine factor due to importance sampling
                let intensity = 50. / PI;

                assert!(intensity >= 0.);

                Color {
                    red: intensity * material.color.red,
                    green: intensity * material.color.green,
                    blue: intensity * material.color.blue,
                }
            } else {
                Color::black()
            }
        } else {
            ambient_occlusion
        }
    } else {
        Color::black()
    }
}

fn color<S: Sampler>(
    ray: Ray,
    scene: &Scene,
    nb_samples: usize,
    sampler: &mut S,
    ambient_occlusion: Color,
) -> Color {
    let mut sum = (0., 0., 0.);

    for _ in 0..nb_samples {
        let color = one_color(ray, scene, sampler.next2d(), ambient_occlusion);

        sum.0 += color.red;
        sum.1 += color.green;
        sum.2 += color.blue;
    }

    let f = 1. / nb_samples as f64;
    Color::new(f * sum.0, f * sum.1, f * sum.2)
}

impl<S: Sampler + 'static> Renderer for MonteCarloRenderer<S> {
    fn render(&self, scene: Scene) {
        let scene = Arc::new(scene);

        let width = scene.camera.width;
        let height = scene.camera.height;
        let iterations_per_pixel = self.iterations_per_step_count;

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

            let ambient_occlusion = self.ambient_occlusion;
            let mut sampler = (self.sampler_factory)(worker_id);

            handles.push(thread::spawn(move || {
                while let Some(request) = rx_worker.recv().unwrap() {
                    sampler.prepare(1, 1, iterations_per_pixel as usize);

                    let (x, y) = (request.x, request.y);

                    // Compute the ray
                    let ray = scene.camera.generate_ray(x, y);

                    // Get the color
                    let color = color(
                        ray,
                        &scene,
                        /*match sampling_method {
                            SamplingMethod::IndependantSamples => {
                                generate_samples_uniform_jitter(iterations_per_pixel)
                            }
                            SamplingMethod::RegularGrid => {
                                generate_samples_regular_grid(iterations_per_pixel)
                            }
                        },*/
                        iterations_per_pixel as usize,
                        &mut sampler,
                        ambient_occlusion,
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
        for step in 0..self.steps_count {
            println!("Iteration {}...", step * self.iterations_per_step_count);

            for x in 0..width {
                for y in 0..height {
                    let worker_id = if x == 0 && y < workers_count {
                        y
                    } else {
                        let answer = rx_main.recv().unwrap();
                        let (x, y) = (answer.x, answer.y);

                        let prev_color = image.pixel(x, y);

                        let step = step as f64;
                        image.set_pixel(
                            x,
                            y,
                            Color::new(
                                (step * prev_color.red + answer.color.red) / (step + 1.),
                                (step * prev_color.green + answer.color.green) / (step + 1.),
                                (step * prev_color.blue + answer.color.blue) / (step + 1.),
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

                let step = step as f64;
                image.set_pixel(
                    x,
                    y,
                    Color::new(
                        (step * prev_color.red + answer.color.red) / (step + 1.),
                        (step * prev_color.green + answer.color.green) / (step + 1.),
                        (step * prev_color.blue + answer.color.blue) / (step + 1.),
                    ),
                );
            }

            image.export(&format!(
                "{}/output-{:0>5}.png",
                self.output_folder,
                self.iterations_per_step_count * (step + 1)
            ));
            image.raw_export(&format!(
                "{}/raw-output-{:0>5}",
                self.output_folder,
                self.iterations_per_step_count * (step + 1)
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
