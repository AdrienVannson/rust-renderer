pub trait Sampler {
    /// Create a new instance of the sampler.
    fn new(seed: u64) -> Self;

    /// Initializes the sampler. It will need to generate a fixed number of samples.
    /// Each sample will be composed of at most a fixed number of unique random values and a fixed
    /// number of couples of random values.
    fn prepare(&mut self, nb_1d: usize, nb_2d: usize, nb_samples: usize);

    /// Start generating a new sample
    fn new_sample(&mut self);

    /// Returns a random value in [0; 1]
    fn next1d(&mut self) -> f64;

    /// Returns a two random values in [0; 1]
    fn next2d(&mut self) -> [f64; 2];
}
