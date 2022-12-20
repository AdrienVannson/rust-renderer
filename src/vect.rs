use std::ops::{Add, Mul, Sub};

/// Vector in space
#[derive(Copy, Clone)]
pub struct Vect {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vect {
    /// Creates a new vector
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Returns the norm of the vector
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalizes the vector
    pub fn normalize(&mut self) {
        let norm = self.norm();
        self.x /= norm;
        self.y /= norm;
        self.z /= norm;
    }

    /// Returns a normalized version of the vector
    pub fn normalized(&self) -> Self {
        let norm = self.norm();
        Vect {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }
}

impl Add for Vect {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vect {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vect {
    type Output = f64;

    // Scalar product
    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<Vect> for f64 {
    type Output = Vect;

    fn mul(self, other: Vect) -> Vect {
        Vect {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
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
