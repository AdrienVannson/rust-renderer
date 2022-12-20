use crate::collision::Collision;
use crate::ray::Ray;

pub trait Shape {
    fn collision_date(&self, ray: Ray) -> f64;
    fn collision(&self, ray: Ray) -> Collision;
}
