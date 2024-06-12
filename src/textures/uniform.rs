use crate::Texture;

/// A uniform texture has the same value everywhere
struct Uniform<T: Clone> {
    val: T,
}

impl<T: Clone> Uniform<T> {
    pub fn new(val: T) -> Self {
        Self { val }
    }
}

impl<T: Clone> Texture<T> for Uniform<T> {
    fn get(&self, uv: (f64, f64)) -> T {
        self.val.clone()
    }
}
