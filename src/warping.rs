use crate::vect::complete_basis_from_1;
use crate::{Transform, Vect};
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

pub fn to_uniform_directed_hemisphere(v: Vect, sample: [f64; 2]) -> Vect {
    let basis = complete_basis_from_1(v);
    let transform = Transform::new_local_to_world(Vect::zero(), basis[1], basis[2], basis[0]);

    transform.apply_vector(to_uniform_hemisphere(sample))
}

pub fn to_cosine_hemisphere(sample: [f64; 2]) -> Vect {
    let on_disk = to_uniform_disk(sample);
    Vect::new(on_disk.x, on_disk.y, (1. - on_disk.squared_norm()).sqrt())
}

pub fn to_cosine_directed_hemisphere(v: Vect, sample: [f64; 2]) -> Vect {
    let basis = complete_basis_from_1(v);
    let transform = Transform::new_local_to_world(Vect::zero(), basis[1], basis[2], basis[0]);

    transform.apply_vector(to_cosine_hemisphere(sample))
}

#[cfg(test)]
mod tests {
    use super::*;
    use fastrand::Rng;

    #[test]
    fn test_uniform_directed_hemisphere() {
        let mut rng = Rng::with_seed(42);

        for _ in 0..100 {
            let sample: [f64; 2] = [rng.f64(), rng.f64()];

            let v = Vect::new(rng.f64() - 0.5, rng.f64() - 0.5, rng.f64() - 0.5).normalized();
            let u = to_uniform_directed_hemisphere(v, sample);

            assert!(u * v >= 0.);
        }
        let mut v = Vect::new(1., -1., 0.);
        let norm = v.norm();

        assert!(1.4142 <= norm && norm <= 1.4143);

        v.normalize();
        assert!(0.99 <= v.norm() && v.norm() <= 1.01);
    }
}
