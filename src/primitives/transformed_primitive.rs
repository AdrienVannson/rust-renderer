use crate::{
    material::Material, primitive::Primitive, ray::Ray, shape::Collision, transform::Transform,
};

/// Applies an affine transformation on a primitive
pub struct TransformedPrimitive {
    primitive: Box<dyn Primitive>,
    object_to_world: Transform, // Object to world
}

impl TransformedPrimitive {
    pub fn new(primitive: Box<dyn Primitive>, object_to_world: Transform) -> Self {
        TransformedPrimitive {
            primitive,
            object_to_world,
        }
    }
}

impl Primitive for TransformedPrimitive {
    fn collision_date(&self, ray: Ray) -> Option<f64> {
        self.primitive.collision_date(self.object_to_world.apply_inv_ray(ray))
    }

    fn collision(&self, ray: Ray) -> Option<Collision> {
        match self.primitive.collision(self.object_to_world.apply_inv_ray(ray)) {
            None => None,
            Some(col) => Some(Collision {
                date: col.date,
                pos: self.object_to_world.apply_point(col.pos),
                normal: self.object_to_world.apply_inv_normal(col.normal).normalized(),
            }),
        }
    }

    fn material_at_collition(&self, collision: Collision) -> Material {
        self.primitive.material_at_collition(collision)
    }
}
