use std::cmp::min;

use crate::{ray::Ray, renderer::Renderer, scene::Scene, vect::Vect};

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
    pub fn render(&self, scene: &Scene, renderer: &dyn Renderer) -> Vec<Vec<(u8, u8, u8)>> {
        let mut image = Vec::new();

        let i = Vect::new(self.dir.y, -self.dir.x, 0.).normalized();
        let j = -(self.dir ^ i).normalized();

        assert!(j.z >= 0.);

        for x in 0..self.width {
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
        }

        image
    }
}
