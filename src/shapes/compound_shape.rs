use crate::ray::Ray;
use crate::shape::{Collision, Shape};

#[derive(Clone, Debug)]
pub struct CompoundShape {
    children: Vec<Box<dyn Shape>>,
}

impl CompoundShape {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            children: Vec::new(),
        })
    }

    pub fn add(&mut self, shape: Box<dyn Shape>) {
        self.children.push(shape);
    }
}

impl Shape for CompoundShape {
    fn collision_date(&self, ray: Ray) -> Option<f64> {
        let mut current_collision = None;

        for child in &self.children {
            current_collision = match (current_collision, child.collision_date(ray)) {
                (None, col) => col,
                (Some(t1), Some(t2)) if t2 < t1 => Some(t2),
                (col, _) => col,
            };
        }

        current_collision
    }

    // TODO there should be a way to store information to avoid recomputing everything
    fn collision(&self, ray: Ray) -> Option<Collision> {
        if let Some(date) = self.collision_date(ray) {
            for child in &self.children {
                if let Some(date_child) = child.collision_date(ray) {
                    if date == date_child {
                        return child.collision(ray);
                    }
                }
            }

            unreachable!()
        } else {
            None
        }
    }
}
