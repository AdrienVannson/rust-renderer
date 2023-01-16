use std::ops::Mul;

use crate::vect::Vect;

pub struct Matrix4x4 {
    /// The entries of the matrix, where m[i][j] is the entry on the i-th line
    /// and the j-th column.
    /// The last line of the matrix is always (0, 0, 0, 1). We don't store it.
    pub m: [[f64; 4]; 3],
}

impl Matrix4x4 {
    /// Returns the identity matrix
    pub fn identity() -> Self {
        Self {
            m: [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]],
        }
    }

    /// Builds a new instance from raw data
    pub fn new(mat: [[f64; 4]; 3]) -> Self {
        Self { m: mat }
    }

    /// Performs a matrix-vector multiplication. The first three components are
    /// the components of v, and the last one is cst.
    pub fn mul(&self, v: Vect, cst: f64) -> Vect {
        let m = &self.m;

        Vect {
            x: m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z + m[0][3] * cst,
            y: m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z + m[1][3] * cst,
            z: m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z + m[2][3] * cst,
        }
    }
}

impl Mul for &Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Self) -> Matrix4x4 {
        let mut m = [[0.; 4]; 3];

        for i in 0..3 {
            m[i][3] = self.m[i][3];
            for j in 0..4 {
                for k in 0..3 {
                    m[i][j] += self.m[i][k] * rhs.m[k][j];
                }
            }
        }

        Matrix4x4 { m }
    }
}
