use crate::Vect;
use std::f64::consts::PI;

pub fn to_uniform_disk(sample: [f64; 2]) -> Vect {
    let r = sample[0].sqrt();
    let phi = 2. * PI * sample[1];

    Vect::new(r * phi.cos(), r * phi.sin(), 0.)
}

pub fn to_uniform_sphere(sample: [f64; 2]) -> Vect {
    let z = 2. * sample[0] - 1.;
    let r = (1. - z * z).sqrt();
    let phi = 2. * PI * sample[1];

    Vect::new(r * phi.cos(), r * phi.sin(), z)
}

pub fn to_uniform_hemisphere(sample: [f64; 2]) -> Vect {
    let z = sample[0];
    let r = (1. - z * z).sqrt();
    let phi = 2. * PI * sample[1];

    Vect::new(r * phi.cos(), r * phi.sin(), z)
}

pub fn to_cosine_hemisphere(sample: [f64; 2]) -> Vect {
    let on_disk = to_uniform_disk(sample);
    Vect::new(on_disk.x, on_disk.y, (1. - on_disk.squared_norm()).sqrt())
}
