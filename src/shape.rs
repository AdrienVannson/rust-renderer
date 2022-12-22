use crate::Vect;
use crate::ray::Ray;

pub trait Shape {
    fn collision_date(&self, ray: Ray) -> Option<f64>;
    fn collision(&self, ray: Ray) -> Option<Collision>;
}

/// Collision between a ray and a shape
pub struct Collision {
    /// The date at which the collision occurs
    pub date: f64,

    /// The position of the collision
    pub pos: Vect,

    /// The normal of the object at the position of collision
    pub normal: Vect,
}