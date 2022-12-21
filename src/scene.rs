use std::thread::current;

use crate::collision::Collision;
use crate::ray::Ray;
use crate::shape::Shape;

pub struct Scene {
    objects: Vec<Box<dyn Shape>>,
}

impl Scene {
    /// Creates a new scene
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    /// Adds an object to the scene
    pub fn add_object(&mut self, obj: Box<dyn Shape>) {
        self.objects.push(obj);
    }

    /// Returns the object colliding with a ray and the information about the collision
    pub fn collision(&self, ray: Ray) -> Option<(&Box<dyn Shape>, Collision)> {
        let mut res = None;

        for obj in self.objects.iter() {
            let current_collision = obj.collision(ray);

            match res {
                None => res = Some((obj, current_collision)),
                Some((_, ref chosen_collision)) => {
                    if current_collision.date < chosen_collision.date {
                        res = Some((obj, current_collision))
                    }
                }
            }
        }

        res
    }
}
