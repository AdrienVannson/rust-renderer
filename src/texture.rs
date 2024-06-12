/// A texture is a function mapping a value to each texture coordinate
pub trait Texture<T> {
    fn get(&self, uv: (f64, f64)) -> T;
}
