use crate::color::Color;
use crate::primitive::Primitive;
use crate::ray::Ray;
use crate::scene::Scene;
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

    fn color(&self, ray: Ray, scene: &Scene) -> Color {
        let collision = self
            .shape
            .collision(ray)
            .expect("the ray should collide the shape");

        Color::new(1., 1., 0.)
    }
}
