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

    /// Returns the value of the n-th component of the vector
    pub fn component(&self, n: u32) -> f64 {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => unreachable!(),
        }
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

/// Generate a basis from a vector of length 1
pub fn complete_basis_from_1(i: Vect) -> [Vect; 3] {
    let j = if i.x.abs() <= 0.5 {
        Vect::new(0., -i.z, i.y)
    } else {
        Vect::new(-i.y, i.x, 0.)
    }.normalized();
    let k = i ^ j;
    [i, j, k]
}

/// Generate a basis from two orthonormal vectors
pub fn complete_basis_from_2(i: Vect, j: Vect) -> [Vect; 3] {
    let k = i ^ j;
    [i, j, k]
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

    #[test]
    fn test_basis_completion() {
        let i = Vect::new(1., 2., -3.).normalized();
        let [i, j, k] = complete_basis_from_1(i);
        println!("{} {} {}", (i-j).norm(), (j - k).norm(), (k-i).norm());
        assert!((i * j).abs() <= 1e-4 && (j * k).abs() <= 1e-4 && (k * i).abs() <= 1e-4);
        assert!(
            (i.norm() - 1.).abs() <= 1e-4
                && (j.norm() - 1.).abs() <= 1e-4
                && (k.norm() - 1.).abs() <= 1e-4
        );

        let i = Vect::new(3., -2., 1.).normalized();
        let [i, j, k] = complete_basis_from_1(i);
        println!("{} {} {}", (i-j).norm(), (j - k).norm(), (k-i).norm());
        assert!((i * j).abs() <= 1e-4 && (j * k).abs() <= 1e-4 && (k * i).abs() <= 1e-4);
        assert!(
            (i.norm() - 1.).abs() <= 1e-4
                && (j.norm() - 1.).abs() <= 1e-4
                && (k.norm() - 1.).abs() <= 1e-4
        );
    }
}
