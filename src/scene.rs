use crate::camera::Camera;
use crate::light::Light;
use crate::primitive::Primitive;
use crate::ray::Ray;
use crate::shape::Collision;

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    primitives: Vec<Box<dyn Primitive>>,
}

impl Scene {
    /// Creates a new scene
    pub fn new(camera: Camera) -> Self {
        Scene {
            camera,
            lights: Vec::new(),
            primitives: Vec::new(),
        }
    }

    /// Adds a light to the scene
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// Adds an object to the scene
    pub fn add_primitive(&mut self, prim: Box<dyn Primitive>) {
        self.primitives.push(prim);
    }

    /// Returns the object colliding with a ray and the information abour the
    /// collision
    pub fn collision(&self, ray: Ray) -> Option<(&Box<dyn Primitive>, Collision)> {
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
            Some((prim, _)) => Some((prim, prim.collision(ray).unwrap())),
        }
    }
}
