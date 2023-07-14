use std::fmt::Debug;

use crate::{
    shape::{Collision, Shape},
    BoundingBox, Ray, Vect,
};

pub trait ImplicitShape: Send + Sync + Debug {
    /// Returns a minoration of the distance between a point and the object.
    /// If the object has an inside, the value is negative for points inside
    /// the object.
    fn estimated_distance(&self, point: Vect) -> f64;

    /// Returns the gradient of the estimated distance at a given point.
    fn grad(&self, point: Vect) -> Vect;
}

impl<T: ImplicitShape + Clone + 'static> Shape for T {
    fn bounding_box(&self) -> BoundingBox {
        // TODO use a smaaller box
        BoundingBox::new_full()
    }

    fn collision_date(&self, ray: Ray) -> Option<f64> {
        let ray_norm_inv = 1. / ray.dir.norm();

        let mut t = 0.;

        while t < 100. {
            let dist = self.estimated_distance(ray.pos_in(t)).abs();

            if dist < 1e-8 {
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
