use crate::color::Color;
use crate::primitive::Primitive;
use crate::ray::Ray;
use crate::shape::Shape;

pub struct GeometricPrimitive {
    shape: Box<dyn Shape>,
}

impl GeometricPrimitive {
    pub fn new(shape: Box<dyn Shape>) -> Self {
        GeometricPrimitive { shape }
    }
}

impl Primitive for GeometricPrimitive {
    fn collision_date(&self, ray: Ray) -> Option<f64> {
        self.shape.collision_date(ray)
    }

    fn color(&self, ray: Ray) -> Color {
        Color::new(1., 1., 0.)
    }
}
