use std::ops::Mul;

pub struct Matrix4x4 {
    /// The entries of the matrix, where m[i][j] is the entry on the i-th line
    /// and the j-th column.
    pub m: [[f64; 4]; 4],
}

impl Matrix4x4 {
    /// Returns the identity matrix
    pub fn new() -> Self {
        Self {
            m: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
}

impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut m = [[0.; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    m[i][j] += self.m[i][k] * rhs.m[k][j];
                }
            }
        }

        Self { m }
    }
}
