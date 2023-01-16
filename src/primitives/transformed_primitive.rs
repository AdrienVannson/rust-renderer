use crate::{
    material::Material, primitive::Primitive, ray::Ray, shape::Collision, transform::Transform,
};

/// Applies an affine transformation on a primitive
pub struct TransformedPrimitive {
    primitive: Box<dyn Primitive>,
    transform: Transform, // Object to world
}

impl TransformedPrimitive {
    pub fn new(primitive: Box<dyn Primitive>, transform: Transform) -> Self {
        TransformedPrimitive {
            primitive,
            transform,
        }
    }
}

impl Primitive for TransformedPrimitive {
    fn collision_date(&self, ray: Ray) -> Option<f64> {
        self.primitive.collision_date(self.transform.apply_ray(ray))
    }

    fn collision(&self, ray: Ray) -> Option<Collision> {
        match self.primitive.collision(self.transform.apply_ray(ray)) {
            None => None,
            Some(col) => Some(Collision {
                date: col.date,
                pos: col.pos,
                normal: col.normal,
            }),
        }
    }

    fn material_at_collition(&self, collision: Collision) -> Material {
        self.primitive.material_at_collition(collision)
    }
}
