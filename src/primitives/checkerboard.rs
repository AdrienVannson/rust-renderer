use crate::color::Color;
use crate::material::Material;
use crate::primitive::Primitive;
use crate::ray::Ray;
use crate::shape::Collision;
use crate::vect::Vect;

/// The checkboard is necessary horizontal
pub struct Checkerboard {
    origin: Vect,
    width: f64,
    height: f64,
    colors: [Color; 2],
}

impl Checkerboard {
    pub fn new(origin: Vect, width: f64, height: f64, color1: Color, color2: Color) -> Self {
        Checkerboard {
            origin,
            width,
            height,
            colors: [color1, color2],
        }
    }
}

impl Primitive for Checkerboard {
    fn collision_date(&self, ray: Ray) -> Option<f64> {
        match self.collision(ray) {
            None => None,
            Some(collision) => Some(collision.date),
        }
    }

    fn collision(&self, ray: Ray) -> Option<Collision> {
        let dist = self.origin.z - ray.pos().z;

        // TODO: check if ray.dir().z < EPSILON
        let collision_date = dist / ray.dir().z;

        if collision_date < 0. {
            return None;
        }

        let pos = ray.pos() + collision_date * ray.dir();

        if self.origin.x <= pos.x
            && pos.x <= self.origin.x + self.width
            && self.origin.y <= pos.y
            && pos.y <= self.origin.y + self.height
        {
            Some(Collision {
                date: collision_date,
                pos,
                normal: Vect::new(0., 0., 1.),
            })
        } else {
            None
        }
    }

    fn material_at_collition(&self, _collision: Collision) -> Material {
        Material {
            color: self.colors[0],
        }
    }
}
