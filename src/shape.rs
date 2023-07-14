use std::fmt::Debug;

use crate::{BoundingBox, Ray, Vect};

pub trait Shape: Send + Sync + Debug + ShapeClone {
    /// Returns a bounding box (idealy the smallest) containing the object
    fn bounding_box(&self) -> BoundingBox;

    /// Returns the date at which a ray will collide with the object. The ray
    /// may start inside the object.
    fn collision_date(&self, ray: Ray) -> Option<f64>;

    // TODO no option
    fn collision(&self, ray: Ray) -> Option<Collision>;
}

pub trait ShapeClone {
    fn clone_box(&self) -> Box<dyn Shape>;
}

impl<T: Shape + Clone + 'static> ShapeClone for T {
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
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
