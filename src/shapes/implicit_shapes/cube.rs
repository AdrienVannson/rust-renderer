use crate::{shapes::implicit_shape::ImplicitShape, vect::Vect};

pub struct Cube {}

impl ImplicitShape for Cube {
    fn estimated_distance(&self, point: Vect) -> f64 {
        point.x.abs().max(point.y.abs()).max(point.z.abs()) - 1.
    }

    /// Returns the gradient of the estimated distance at a given point.
    fn grad(&self, point: Vect) -> Vect {
        let x_abs = point.x.abs();
        let y_abs = point.y.abs();
        let z_abs = point.z.abs();

        if x_abs >= y_abs && x_abs >= z_abs {
            Vect::new(point.x.signum(), 0., 0.)
        } else if y_abs >= x_abs && y_abs >= z_abs {
            Vect::new(0., point.y.signum(), 0.)
        } else {
            Vect::new(0., 0., point.z.signum())
        }
    }
}
