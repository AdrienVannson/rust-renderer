use crate::{Collision, Material, Primitive, Ray, Shape};

pub struct GeometricPrimitive {
    shape: Box<dyn Shape>,
    material: Material,
}

impl GeometricPrimitive {
    pub fn new(shape: Box<dyn Shape>, material: Material) -> Self {
        GeometricPrimitive { shape, material }
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
        self.material
    }
}
