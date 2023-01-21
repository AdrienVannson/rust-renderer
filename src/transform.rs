use crate::{matrix::Matrix4x4, ray::Ray, vect::Vect};

pub struct Transform {
    mat: Matrix4x4,
    mat_inv: Matrix4x4,
}

impl Transform {
    /// Returns the identity transformation
    pub fn new_identity() -> Self {
        Transform {
            mat: Matrix4x4::identity(),
            mat_inv: Matrix4x4::identity(),
        }
    }

    /// Returns a translation
    pub fn new_translation(v: Vect) -> Self {
        let mat = Matrix4x4::new([[1., 0., 0., v.x], [0., 1., 0., v.y], [0., 0., 1., v.z]]);
        let mat_inv = Matrix4x4::new([[1., 0., 0., -v.x], [0., 1., 0., -v.y], [0., 0., 1., -v.z]]);
        Self { mat, mat_inv }
    }

    /// Returns a uniform scaling
    pub fn new_uniform_scaling(s: f64) -> Self {
        Self::new_scaling(s, s, s)
    }

    /// Returns a scaling
    pub fn new_scaling(sx: f64, sy: f64, sz: f64) -> Self {
        let mat = Matrix4x4::new([[sx, 0., 0., 0.], [0., sy, 0., 0.], [0., 0., sz, 0.]]);
        let mat_inv = Matrix4x4::new([
            [1. / sx, 0., 0., 0.],
            [0., 1. / sy, 0., 0.],
            [0., 0., 1. / sz, 0.],
        ]);
        Self { mat, mat_inv }
    }

    /// Returns a rotation around the x axis
    pub fn new_x_rotation(theta: f64) -> Self {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        let mat = Matrix4x4::new([
            [1., 0., 0., 0.],
            [0., cos_theta, -sin_theta, 0.],
            [0., sin_theta, cos_theta, 0.]
        ]);
        let mat_inv = Matrix4x4::new([
            [1., 0., 0., 0.],
            [0., cos_theta, sin_theta, 0.],
            [0., -sin_theta, cos_theta, 0.]
        ]);
        Self { mat, mat_inv }
    }

    /// Returns a rotation around the y axis
    pub fn new_y_rotation(theta: f64) -> Self {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        let mat = Matrix4x4::new([
            [cos_theta, 0., sin_theta, 0.],
            [0., 1., 0., 0.],
            [-sin_theta, 0., cos_theta, 0.]
        ]);
        let mat_inv = Matrix4x4::new([
            [cos_theta, 0., -sin_theta, 0.],
            [0., 1., 0., 0.],
            [sin_theta, 0., cos_theta, 0.]
        ]);
        Self { mat, mat_inv }
    }

    /// Returns a rotation around the z axis
    pub fn new_z_rotation(theta: f64) -> Self {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        let mat = Matrix4x4::new([
            [cos_theta, -sin_theta, 0., 0.],
            [sin_theta, cos_theta, 0., 0.],
            [0., 0., 1., 0.]
        ]);
        let mat_inv = Matrix4x4::new([
            [cos_theta, sin_theta, 0., 0.],
            [-sin_theta, cos_theta, 0., 0.],
            [0., 0., 1., 0.]
        ]);
        Self { mat, mat_inv }
    }

    /// Returns the transformation obtained when applying other after the
    /// current tranformation
    pub fn add(&self, other: &Transform) -> Self {
        Self {
            mat: &other.mat * &self.mat,
            mat_inv: &self.mat_inv * &other.mat_inv
        }
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

        // The matrix is transposed
        Vect {
            x: m[0][0] * v.x + m[1][0] * v.y + m[2][0] * v.z,
            y: m[0][1] * v.x + m[1][1] * v.y + m[2][1] * v.z,
            z: m[0][2] * v.x + m[1][2] * v.y + m[2][2] * v.z,
        }
    }

    /// Applies the inverse transformation to a normal
    pub fn apply_inv_normal(&self, v: Vect) -> Vect {
        let m = &self.mat.m;

        // The matrix is transposed
        Vect {
            x: m[0][0] * v.x + m[1][0] * v.y + m[2][0] * v.z,
            y: m[0][1] * v.x + m[1][1] * v.y + m[2][1] * v.z,
            z: m[0][2] * v.x + m[1][2] * v.y + m[2][2] * v.z,
        }
    }

    /// Applies the transformation to a ray
    pub fn apply_ray(&self, ray: Ray) -> Ray {
        Ray {
            pos: self.apply_point(ray.pos),
            dir: self.apply_vector(ray.dir),
        }
    }

    /// Applies the inverse transformation to a ray
    pub fn apply_inv_ray(&self, ray: Ray) -> Ray {
        Ray {
            pos: self.apply_inv_point(ray.pos),
            dir: self.apply_inv_vector(ray.dir),
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
