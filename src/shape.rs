use crate::ray::Ray;
use crate::vect::Vect;

pub trait Shape: Send + Sync {
    /// Returns the date at which a ray will collide with the object. The ray
    /// may start inside the object.
    fn collision_date(&self, ray: Ray) -> Option<f64>;

    // TODO no option
    fn collision(&self, ray: Ray) -> Option<Collision>;
}

impl dyn Shape {
    /// Tests if the origin of the ray is inside the shape
    pub fn ray_starts_inside(&self, ray: Ray) -> bool {
        match self.collision(ray) {
            None => false,
            Some(col) => col.normal * ray.dir > 0.,
        }
    }
}

/// Collision between a ray and a shape
#[derive(Copy, Clone)]
pub struct Collision {
    /// The date at which the collision occurs
    pub date: f64,

    /// The position of the collision
    pub pos: Vect,

    /// The normal of the object at the position of collision. If the object has
    /// an inside and an outside, the normal must point towards the outside.
    /// This is necessary to test whever a point is inside or outside the object.
    pub normal: Vect,
}
