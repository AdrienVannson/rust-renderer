/// Vector in space
struct Vect {
    x: f64,
    y: f64,
    z: f64,
}

impl Vect {
    /// Creates a new vector
    pub fn new(x: f64, y: f64, z: f64) -> Vect {
        Vect { x, y, z }
    }

    /// Returns the norm of the vector
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm() {
        let v = Vect::new(1., -1., 0.);
        let norm = v.norm();

        assert!(1.4142 <= norm && norm <= 1.4143);
    }
}
