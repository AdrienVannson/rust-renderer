use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Collision;

/// A primitive is something that can be renderered.
pub trait Primitive {
    fn collision_date(&self, ray: Ray) -> Option<f64>;
    fn collision(&self, ray: Ray) -> Collision;

    fn material_at_collition(&self, collision: Collision) -> Material;
}
