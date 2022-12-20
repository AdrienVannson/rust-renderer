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
}