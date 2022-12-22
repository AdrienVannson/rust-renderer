use crate::collision::Collision;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::vect::Vect;

pub struct Sphere {
    center: Vect,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vect, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Shape for Sphere {
    fn collision_date(&self, ray: Ray) -> f64 {
        let u = self.center - ray.pos();
        let v = ray.dir();

        let delta = (u * v) * (u * v) + (self.radius * self.radius - u * u) * v * v;

        if delta <= 0. {
            return f64::INFINITY;
        }

        let root1 = (u * v - delta.sqrt()) / (v * v);
        if root1 > 0. {
            return root1;
        }

        let root2 = (u * v + delta.sqrt()) / (v * v);
        if root2 > 0. {
            return root2;
        }

        return f64::INFINITY;
    }

    fn collision(&self, ray: Ray) -> Collision {
        let collision_date = self.collision_date(ray);
        let pos = ray.pos() + collision_date * ray.dir();

        let normal = (pos - self.center).normalized();

        Collision {
            date: collision_date,
            pos,
            normal,
        }
    }
}
