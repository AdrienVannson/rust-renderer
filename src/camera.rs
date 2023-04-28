use std::cmp::min;
use std::{
    sync::{mpsc, Arc},
    thread,
};

use crate::{Color, Ray, Renderer, Scene, Vect};

pub struct Camera {
    // Position of the focus point
    pub pos: Vect,

    // Direction where the camera looks. The norm of the vector is the focal
    // distance of the camera.
    pub dir: Vect,

    // Dimensions of the image
    pub width: usize,
    pub height: usize,
}

impl Camera {
    /// Returns the final image
    pub fn render(
        &self,
        scene: &'static Scene,
        renderer: &'static (dyn Renderer),
    ) -> Vec<Vec<(u8, u8, u8)>> {
        let mut image: Vec<Vec<(u8, u8, u8)>> = vec![vec![(0, 0, 0); self.height]; self.width];

        let i = Vect::new(self.dir.y, -self.dir.x, 0.).normalized();
        let j = -(self.dir ^ i).normalized();

        assert!(j.z >= 0.);

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

        let width = self.width;
        let height = self.height;
        let dir = self.dir;
        let pos = self.pos;
        let ratio = min(self.width, self.height) as f64;
        //let renderer_ = (*renderer).clone();

        let (tx_main, rx_main) = mpsc::channel::<Answer>();

        let renderer = Arc::new(renderer);
        let scene = Arc::new(scene);

        let mut tx_workers = Vec::new();
        let mut handles = Vec::new();

        let workers_count = 4;

        for worker_id in 0..workers_count {
            let renderer = Arc::clone(&renderer);
            let scene = Arc::clone(&scene);
            let (tx_worker, rx_worker) = mpsc::channel::<Option<Request>>();
            tx_workers.push(tx_worker);
            let tx_main = tx_main.clone();

            handles.push(thread::spawn(move || {
                while let Some(request) = rx_worker.recv().unwrap() {
                    let (x, y) = (request.x, request.y);

                    // Compute the ray
                    let dir = dir
                        + ((x as f64) - 0.5 * (width as f64)) / ratio * i
                        + ((y as f64) - 0.5 * (height as f64)) / ratio * j;

                    let ray = Ray::new(pos, dir);

                    // Get the color
                    let color = renderer.color(ray, &scene);

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

        for x in 0..self.width {
            for y in 0..self.height {
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
