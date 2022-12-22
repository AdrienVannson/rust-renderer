use crate::color::Color;
use crate::primitive::Primitive;
use crate::ray::Ray;
use crate::shape::Shape;

pub struct GeometricPrimitive {}

impl Primitive for GeometricPrimitive {
    fn color(&self, ray: Ray) -> Color {
        Color::new(1., 0., 0.)
    }
}
