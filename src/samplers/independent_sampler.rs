use crate::sampler::Sampler;
use fastrand::Rng;

pub struct IndependentSampler {
    rng: Rng,
}

impl IndependentSampler {
    fn new(seed: u64) -> Self {
        IndependentSampler {
            rng: fastrand::Rng::with_seed(seed),
        }
    }
}

impl Sampler for IndependentSampler {
    fn prepare(&mut self, nb_1d: usize, nb_2d: usize, nb_samples: usize) {}

    fn new_sample(&mut self) {}

    fn next1d(&mut self) -> f64 {
        self.rng.f64()
    }

    fn next2d(&mut self) -> [f64; 2] {
        [self.rng.f64(), self.rng.f64()]
    }
}
