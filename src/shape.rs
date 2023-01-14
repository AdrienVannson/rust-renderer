use crate::ray::Ray;
use crate::vect::Vect;

pub trait Shape {
    fn collision_date(&self, ray: Ray) -> Option<f64>;

    // TODO no option
    fn collision(&self, ray: Ray) -> Option<Collision>;
}

/// Collision between a ray and a shape
#[derive(Copy, Clone)]
pub struct Collision {
    /// The date at which the collision occurs
    pub date: f64,

    /// The position of the collision
    pub pos: Vect,

    /// The normal of the object at the position of collision
    pub normal: Vect,
}
