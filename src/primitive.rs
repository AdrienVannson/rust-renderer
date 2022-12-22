use crate::ray::Ray;
use crate::color::Color;

/// A primitive is something that can be renderered.
pub trait Primitive {
    fn color(&self, ray: Ray) -> Color;
}