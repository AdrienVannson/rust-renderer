use crate::vect::Vect;

#[derive(Clone, Debug)]
pub struct Light {
    pub pos: Vect,
    pub intensity: f64,
}
