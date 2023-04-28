use std::ops::{Add, AddAssign, BitXor, Mul, Neg, Sub};

/// Vector in space
#[derive(Copy, Clone, Debug)]
pub struct Vect {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vect {
    /// Create the vector (0, 0, 0)
    pub fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    /// Creates a new vector
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Returns the norm of the vector
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns the squared norm of the vector
    pub fn squared_norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Normalizes the vector
    pub fn normalize(&mut self) {
        let norm_inverse = 1. / self.norm();
        self.x *= norm_inverse;
        self.y *= norm_inverse;
        self.z *= norm_inverse;
    }

    /// Returns a normalized version of the vector
    pub fn normalized(&self) -> Self {
        let norm_inverse = 1. / self.norm();
        Vect {
            x: self.x * norm_inverse,
            y: self.y * norm_inverse,
            z: self.z * norm_inverse,
        }
    }
}

impl Add for Vect {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vect {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vect {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vect {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul for Vect {
    type Output = f64;

    // Scalar product
    fn mul(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<Vect> for f64 {
    type Output = Vect;

    fn mul(self, rhs: Vect) -> Vect {
        Vect {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

// Cross product
impl BitXor for Vect {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Vect {
        Vect {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm() {
        let mut v = Vect::new(1., -1., 0.);
        let norm = v.norm();

        assert!(1.4142 <= norm && norm <= 1.4143);

        v.normalize();
        assert!(0.99 <= v.norm() && v.norm() <= 1.01);
    }
}
