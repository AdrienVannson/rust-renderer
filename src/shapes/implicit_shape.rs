use crate::{
    ray::Ray,
    shape::{Collision, Shape},
    vect::Vect,
};

pub trait ImplicitShape {
    /// Returns a minoration of the distance between a point and the object.
    fn estimated_distance(&self, point: Vect) -> f64;

    /// Returns the gradient of the estimated distance at a given point.
    fn grad(&self, point: Vect) -> Vect;
}

impl<T: ImplicitShape> Shape for T {
    fn collision_date(&self, ray: Ray) -> Option<f64> {
        let ray_norm_inv = 1. / ray.dir.norm();

        let mut t = 0.;

        while t < 100. {
            let dist = self.estimated_distance(ray.pos_in(t));

            if dist < 1e-5 {
                return Some(t);
            }

            t += dist * ray_norm_inv;
        }

        None
    }

    fn collision(&self, ray: Ray) -> Option<Collision> {
        if let Some(date) = self.collision_date(ray) {
            let pos = ray.pos_in(date);

            Some(Collision {
                date,
                pos,
                normal: self.grad(pos).normalized(),
            })
        } else {
            None
        }
    }
}
