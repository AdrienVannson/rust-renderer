use crate::{matrix::Matrix4x4, vect::Vect};

pub struct Transform {
    mat: Matrix4x4,
    mat_inv: Matrix4x4,
}

impl Transform {
    /// Returns the identity transformation
    pub fn new() -> Self {
        Transform {
            mat: Matrix4x4::new(),
            mat_inv: Matrix4x4::new(),
        }
    }

    /// Applies the transformation to a vector
    pub fn apply_vector(&self, v: Vect) -> Vect {
        // Do not apply translations to vectors
        self.mat.mul(v, 0.)
    }

    /// Applies the transformation to a point
    pub fn apply_point(&self, v: Vect) -> Vect {
        // Apply translations to points
        self.mat.mul(v, 1.)
    }
}

/// Returns a transform corresponding to applying t1, then t2
pub fn merge(t1: &Transform, t2: &Transform) -> Transform {
    Transform {
        mat: &t2.mat * &t1.mat,
        mat_inv: &t1.mat_inv * &t2.mat_inv,
    }
}
