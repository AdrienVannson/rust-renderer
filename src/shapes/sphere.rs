use crate::shape::{Collision, Shape};
use crate::{BoundingBox, Ray, Vect};

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Vect,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vect, radius: f64) -> Box<Self> {
        Box::new(Self { center, radius })
    }
}

impl Shape for Sphere {
    fn bounding_box(&self) -> BoundingBox {
        let v = Vect::new(self.radius, self.radius, self.radius);
        BoundingBox::new_from_extremities(self.center - v, self.center + v)
    }

    fn collision_date(&self, ray: Ray) -> Option<f64> {
        let u = self.center - ray.pos();
        let v = ray.dir();

        let delta = (u * v) * (u * v) + (self.radius * self.radius - u * u) * v * v;

        if delta <= 0. {
            return None;
        }

        let root1 = (u * v - delta.sqrt()) / (v * v);
        if root1 > 0. {
            return Some(root1);
        }

        let root2 = (u * v + delta.sqrt()) / (v * v);
        if root2 > 0. {
            return Some(root2);
        }

        return None;
    }

    fn collision(&self, ray: Ray) -> Option<Collision> {
        if let Some(collision_date) = self.collision_date(ray) {
            let pos = ray.pos() + collision_date * ray.dir();

            let normal = (pos - self.center).normalized();

            Some(Collision {
                date: collision_date,
                pos,
                normal,
            })
        } else {
            None
        }
    }
}
