use crate::{Collision, Material, Primitive, Ray, Shape};

#[derive(Clone, Debug)]
pub struct GeometricPrimitive {
    shape: Box<dyn Shape>,
    material: Material,
}

impl GeometricPrimitive {
    pub fn new(shape: Box<dyn Shape>, material: Material) -> Box<Self> {
        Box::new(GeometricPrimitive { shape, material })
    }
}

impl Primitive for GeometricPrimitive {
    fn collision_date(&self, ray: Ray) -> Option<f64> {
        self.shape.collision_date(ray)
    }

    fn collision(&self, ray: Ray) -> Option<Collision> {
        self.shape.collision(ray)
    }

    fn material_at_collision(&self, _collision: Collision) -> Material {
        self.material
    }
}
