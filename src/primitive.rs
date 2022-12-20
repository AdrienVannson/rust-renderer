use crate::collision::Collision;
use crate::ray::Ray;

trait Primitive {
    fn collision_date(&self, ray: Ray) -> f64;
    fn collision(&self, ray: Ray) -> Collision;
}
