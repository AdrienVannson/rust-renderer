use crate::color::Color;
use crate::material::Material;
use crate::primitive::Primitive;
use crate::ray::Ray;
use crate::shape::{Collision, Shape};

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

    fn collision(&self, ray: Ray) -> Option<Collision> {
        self.shape.collision(ray)
    }

    fn material_at_collition(&self, _collision: Collision) -> Material {
        Material {
            color: Color::new(1., 1., 0.),
        }
    }
}
