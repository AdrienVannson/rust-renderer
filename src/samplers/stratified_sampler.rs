use crate::sampler::Sampler;
use fastrand::Rng;

pub struct StratifiedSampler {
    rng: Rng,
    with_jittering: bool,
    samples_2d: Vec<Vec<[f64; 2]>>, // (nb_samples, nb_2d)
    current_sample: usize,
    current_2d_dim: usize,
}

impl StratifiedSampler {
    pub fn new(seed: u64) -> Self {
        StratifiedSampler {
            rng: Rng::with_seed(seed),
            with_jittering: true,
            samples_2d: Vec::new(),
            current_sample: 0,
            current_2d_dim: 0,
        }
    }

    pub fn new_without_jittering(seed: u64) -> Self {
        StratifiedSampler {
            rng: fastrand::Rng::with_seed(seed),
            with_jittering: false,
            samples_2d: Vec::new(),
            current_sample: 0,
            current_2d_dim: 0,
        }
    }
}

impl Sampler for StratifiedSampler {
    fn prepare(&mut self, nb_1d: usize, nb_2d: usize, nb_samples: usize) {
        // TODO 1D
        // TODO several 2d dimensions
        assert_eq!(nb_2d, 1);

        let root = (nb_samples as f64).sqrt() as usize;
        assert_eq!(root * root, nb_samples);

        let mut samples_2d = Vec::new();

        for i in 0..root {
            for j in 0..root {
                // Not 0.5 to prevent rays from being parallel to the walls
                // TODO 0.5
                let (dx, dy) = if self.with_jittering {
                    (self.rng.f64(), self.rng.f64())
                } else {
                    (0.501, 0.501)
                };

                samples_2d.push(vec![[
                    (i as f64 + dx) / root as f64,
                    (j as f64 + dy) / root as f64,
                ]]);
            }
        }

        self.samples_2d = samples_2d;
        self.current_sample = 0;
        self.current_2d_dim = 0;
    }

    fn new_sample(&mut self) {
        if self.current_2d_dim != 0 {
            // TODO and 1D dimension
            self.current_sample += 1;
            self.current_2d_dim = 0;
        }
    }

    fn next1d(&mut self) -> f64 {
        unimplemented!();
    }

    fn next2d(&mut self) -> [f64; 2] {
        self.current_2d_dim += 1;
        self.samples_2d[self.current_sample][self.current_2d_dim - 1]
    }
}
