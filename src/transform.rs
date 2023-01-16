use crate::{matrix::Matrix4x4, vect::Vect};

pub struct Transform {
    mat: Matrix4x4,
    mat_inv: Matrix4x4,
}

impl Transform {
    /// Returns the identity transformation
    pub fn new() -> Self {
        Transform {
            mat: Matrix4x4::identity(),
            mat_inv: Matrix4x4::identity(),
        }
    }

    /// Returns a translation
    pub fn new_translation(v: Vect) -> Self {
        let mat = Matrix4x4::new([[0., 0., 0., v.x], [0., 0., 0., v.y], [0., 0., 0., v.z]]);
        let mat_inv = Matrix4x4::new([[0., 0., 0., -v.x], [0., 0., 0., -v.y], [0., 0., 0., -v.z]]);
        Self { mat, mat_inv }
    }

    /// Applies the transformation to a vector
    pub fn apply_vector(&self, v: Vect) -> Vect {
        // Do not apply translations to vectors
        self.mat.mul(v, 0.)
    }

    /// Applies the inverse transformation to a vector
    pub fn apply_inv_vector(&self, v: Vect) -> Vect {
        // Do not apply translations to vectors
        self.mat_inv.mul(v, 0.)
    }

    /// Applies the transformation to a point
    pub fn apply_point(&self, v: Vect) -> Vect {
        // Apply translations to points
        self.mat.mul(v, 1.)
    }

    /// Applies the inverse transformation to a point
    pub fn apply_inv_point(&self, v: Vect) -> Vect {
        // Apply translations to points
        self.mat_inv.mul(v, 1.)
    }

    /// Applies the transformation to a normal
    pub fn apply_normal(&self, v: Vect) -> Vect {
        let m = &self.mat_inv.m;

        Vect {
            x: m[0][0] * v.x + m[1][0] * v.y + m[2][0] * v.z,
            y: m[0][1] * v.x + m[1][1] * v.y + m[2][1] * v.z,
            z: m[0][2] * v.x + m[1][2] * v.y + m[2][2] * v.z,
        }
    }

    /// Applies the inverse transformation to a normal
    pub fn apply_inv_normal(&self, v: Vect) -> Vect {
        let m = &self.mat.m;

        Vect {
            x: m[0][0] * v.x + m[1][0] * v.y + m[2][0] * v.z,
            y: m[0][1] * v.x + m[1][1] * v.y + m[2][1] * v.z,
            z: m[0][2] * v.x + m[1][2] * v.y + m[2][2] * v.z,
        }
    }
}

/// Returns a transform corresponding to applying t1, then t2
pub fn merge(t1: &Transform, t2: &Transform) -> Transform {
    Transform {
        mat: &t2.mat * &t1.mat,
        mat_inv: &t1.mat_inv * &t2.mat_inv,
    }
}
