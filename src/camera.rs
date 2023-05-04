use std::cmp::min;

use crate::{Ray, Vect};

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
    pub fn generate_ray(&self, x: usize, y: usize) -> Ray {
        let ratio = min(self.width, self.height) as f64;

        let i = Vect::new(self.dir.y, -self.dir.x, 0.).normalized();
        let j = -(self.dir ^ i).normalized();
        assert!(j.z >= 0.);

        let dir = self.dir
            + ((x as f64) - 0.5 * (self.width as f64)) / ratio * i
            + ((y as f64) - 0.5 * (self.height as f64)) / ratio * j;

        Ray::new(self.pos, dir)
    }
}
