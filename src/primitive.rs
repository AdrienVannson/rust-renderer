use crate::color::Color;
use crate::ray::Ray;

/// A primitive is something that can be renderered.
pub trait Primitive {
    fn collision_date(&self, ray: Ray) -> Option<f64>;

    fn color(&self, ray: Ray) -> Color;
}
