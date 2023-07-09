use crate::ray::Ray;
use crate::shape::{Collision, Shape};
use crate::vect::Vect;

pub struct Triangle {
    a: Vect,
    b: Vect,
    c: Vect,
}

impl Triangle {
    pub fn new(a: Vect, b: Vect, c: Vect) -> Box<Self> {
        Box::new(Self { a, b, c })
    }
}

impl Shape for Triangle {
    fn collision_date(&self, ray: Ray) -> Option<f64> {
        let n = (self.b - self.a) ^ (self.c - self.a);

        let lambda = (n * (self.a - ray.pos)) / (n * ray.dir);

        if lambda <= 0. {
            return None;
        }

        let m = ray.pos() + lambda * ray.dir();

        // Check if M is in ABC
        if ((self.b - self.a) ^ (m - self.a)) * ((m - self.a) ^ (self.c - self.a)) >= 0.
            && ((self.a - self.b) ^ (m - self.b)) * ((m - self.b) ^ (self.c - self.b)) >= 0.
            && ((self.a - self.c) ^ (m - self.c)) * ((m - self.c) ^ (self.b - self.c)) >= 0.
        {
            Some(lambda)
        } else {
            None
        }
    }

    fn collision(&self, ray: Ray) -> Option<Collision> {
        let normal = ((self.b - self.a) ^ (self.c - self.a)).normalized();

        match self.collision_date(ray) {
            None => None,
            Some(date) => Some(Collision {
                date,
                pos: ray.pos_in(date),
                normal,
            }),
        }
    }
}
