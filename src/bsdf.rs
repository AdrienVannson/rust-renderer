use crate::{Color, Vect};

pub trait BSDF {
    /// Given wi and wo
    fn eval(wi: Vect, wo: Vect) -> Color;

    fn pdf(wi: Vect, wo: Vect) -> f64;
}
