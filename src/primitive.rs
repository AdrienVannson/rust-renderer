use std::fmt::Debug;

use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Collision;

/// A primitive is something that can be rendered.
pub trait Primitive: Send + Sync + PrimitiveClone + Debug {
    fn collision_date(&self, ray: Ray) -> Option<f64>;
    fn collision(&self, ray: Ray) -> Option<Collision>;

    fn material_at_collision(&self, collision: Collision) -> Material;
}

pub trait PrimitiveClone {
    fn clone_box(&self) -> Box<dyn Primitive>;
}

impl<T: Primitive + Clone + 'static> PrimitiveClone for T {
    fn clone_box(&self) -> Box<dyn Primitive> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Primitive> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
