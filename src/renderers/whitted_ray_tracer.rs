use std::{
    sync::{mpsc, Arc},
    thread,
    thread::available_parallelism,
};

use crate::{Color, Image, Ray, Renderer, Scene};

pub struct WhittedRayTracer {}

fn color(ray: Ray, scene: &Arc<Scene>) -> Color {
    if let Some((primitive, collision)) = scene.collision(ray) {
        let material = primitive.material_at_collision(collision);

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

impl Renderer for WhittedRayTracer {
    fn render(&self, scene: Scene) {
        let scene = Arc::new(scene);

        let width = scene.camera.width;
        let height = scene.camera.height;

        let mut image = Image::new(width, height);

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

        let workers_count = available_parallelism().unwrap().get();

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
                    let color = color(ray, &scene);

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

                    image.set_pixel(x, y, answer.color);

                    answer.sender
                };

                tx_workers[worker_id].send(Some(Request { x, y })).unwrap();
            }
        }

        for _ in 0..workers_count {
            let answer = rx_main.recv().unwrap();
            let (x, y) = (answer.x, answer.y);

            image.set_pixel(x, y, answer.color);
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
