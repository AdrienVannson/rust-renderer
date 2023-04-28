use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::{sync::mpsc, thread};

use crate::{Color, Ray, Renderer, Scene, Vect};

pub struct Camera {
    // Position of the focus point
    pub pos: Vect,

    // Direction where the camera looks. The norm of the vector is the focal
    // distance of the camera.
    pub dir: Vect,

    // Dimensions of the image
    pub width: u32,
    pub height: u32,
}

impl Camera {
    /// Returns the final image
    pub fn render(
        &self,
        scene: &'static Scene,
        renderer: &'static (dyn Renderer),
    ) -> Vec<Vec<(u8, u8, u8)>> {
        let mut image = Vec::<Vec<(u8, u8, u8)>>::new();

        let i = Vect::new(self.dir.y, -self.dir.x, 0.).normalized();
        let j = -(self.dir ^ i).normalized();

        assert!(j.z >= 0.);

        /*for x in 0..self.width {
            let mut column = Vec::new();

            for y in 0..self.height {
                let ratio = min(self.width, self.height) as f64;

                // Compute the ray
                let dir = self.dir
                    + ((x as f64) - 0.5 * (self.width as f64)) / ratio * i
                    + ((y as f64) - 0.5 * (self.height as f64)) / ratio * j;

                let ray = Ray::new(self.pos, dir);

                // Get the color
                //let color = scene.color(ray, 3);
                let color = renderer.color(ray, scene);

                // Compute the pixel
                let pixel = (
                    (255. * color.red) as u8,
                    (255. * color.green) as u8,
                    (255. * color.blue) as u8,
                );

                column.push(pixel);
            }

            image.push(column);
        }*/

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

        {
            let renderer = Arc::clone(&renderer);
            let scene = Arc::clone(&scene);
            let (tx_worker, rx_worker) = mpsc::channel::<Request>();

            let handle = thread::spawn(move || {
                let request = rx_worker.recv().unwrap();

                let (x, y) = (request.x, request.y);

                // Compute the ray
                let dir = dir
                    + ((x as f64) - 0.5 * (width as f64)) / ratio * i
                    + ((y as f64) - 0.5 * (height as f64)) / ratio * j;

                let ray = Ray::new(pos, dir);

                // Get the color
                renderer.color(ray, &scene);
            });

            handle.join().unwrap();
        }

        {
            let renderer = Arc::clone(&renderer);
            let scene = Arc::clone(&scene);
            let (tx_worker, rx_worker) = mpsc::channel::<Request>();

            let handle = thread::spawn(move || {
                let request = rx_worker.recv().unwrap();

                let (x, y) = (request.x, request.y);

                // Compute the ray
                let dir = dir
                    + ((x as f64) - 0.5 * (width as f64)) / ratio * i
                    + ((y as f64) - 0.5 * (height as f64)) / ratio * j;

                let ray = Ray::new(pos, dir);

                // Get the color
                renderer.color(ray, &scene);
            });

            handle.join().unwrap();
        }

        image
    }
}
