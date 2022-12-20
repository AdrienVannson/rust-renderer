use crate::primitive::Primitive;

pub struct Scene {
    objects: Vec<Box<dyn Primitive>>,
}

impl Scene {
    /// Creates a new scene
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    /// Adds an object to the scene
    pub fn add_object(&mut self, obj: Box<dyn Primitive>) {
        self.objects.push(obj);
    }
}