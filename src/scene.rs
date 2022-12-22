use crate::camera::Camera;
use crate::color::Color;
use crate::primitive::Primitive;
use crate::ray::Ray;

pub struct Scene {
    pub camera: Camera,
    primitives: Vec<Box<dyn Primitive>>,
}

impl Scene {
    /// Creates a new scene
    pub fn new(camera: Camera) -> Self {
        Scene {
            camera,
            primitives: Vec::new(),
        }
    }

    /// Adds an object to the scene
    pub fn add_primitive(&mut self, prim: Box<dyn Primitive>) {
        self.primitives.push(prim);
    }

    /// Returns the object colliding with a ray
    fn collision(&self, ray: Ray) -> Option<&Box<dyn Primitive>> {
        let mut earliest_collision: Option<(&Box<dyn Primitive>, f64)> = None;

        for prim in self.primitives.iter() {
            if let Some(collision_date) = prim.collision_date(ray) {
                let earliest_date = match earliest_collision {
                    None => f64::INFINITY,
                    Some((_, date)) => date,
                };

                if collision_date < earliest_date {
                    earliest_collision = Some((prim, collision_date));
                }
            }
        }

        match earliest_collision {
            None => None,
            Some((prim, _)) => Some(prim),
        }
    }

    pub fn color(&self, ray: Ray, remaining_depth: i32) -> Color {
        if remaining_depth == 0 {
            return Color::black();
        }

        if let Some(prim) = self.collision(ray) {
            prim.color(ray, self)
        } else {
            Color::black() // Background color
        }
    }
}
