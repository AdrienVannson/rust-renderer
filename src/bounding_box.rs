use std::ops::{Add, Mul};

use crate::{Ray, Vect};

#[derive(Clone, Debug)]
pub struct BoundingBox {
    // If None, the box is empty.
    // Otherwise, the two vertices are the vertices with the minimal and the maximal coordinates.
    extremities: Option<(Vect, Vect)>,
}

impl BoundingBox {
    /// Returns an empty bounding box.
    pub fn new() -> Self {
        Self { extremities: None }
    }

    /// Returns a box containing the whole space.
    pub fn new_full() -> Self {
        Self {
            extremities: Some((
                Vect::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY),
                Vect::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            )),
        }
    }

    pub fn new_from_extremities(min: Vect, max: Vect) -> Self {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        assert!(min.z <= max.z);

        Self {
            extremities: Some((min, max)),
        }
    }

    pub fn add_point(&mut self, point: Vect) {
        if let Some((min, max)) = &mut self.extremities {
            min.x = f64::min(point.x, min.x);
            min.y = f64::min(point.y, min.y);
            min.z = f64::min(point.z, min.z);

            max.x = f64::max(point.x, max.x);
            max.y = f64::max(point.y, max.y);
            max.z = f64::max(point.z, max.z);
        } else {
            self.extremities = Some((point, point));
        }
    }

    pub fn collision_date(&self, ray: Ray) -> Option<f64> {
        if let Some((min, max)) = self.extremities {
            let mut t_min = -f64::INFINITY;
            let mut t_max = f64::INFINITY;

            for i in 0..3 {
                // TODO check if the component is near 0 before inverting
                let t1 = (min.component(i) - ray.pos.component(i)) / ray.dir.component(i);
                let t2 = (max.component(i) - ray.pos.component(i)) / ray.dir.component(i);

                t_min = f64::max(f64::min(t1, t2), t_min);
                t_max = f64::min(f64::max(t1, t2), t_max);
            }

            if t_min <= t_max && t_max >= 0. {
                Some(f64::max(t_min, 0.))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Add for &BoundingBox {
    type Output = BoundingBox;

    /// Returns the union of the two bounding boxes
    fn add(self, rhs: Self) -> BoundingBox {
        match (self.extremities, rhs.extremities) {
            (None, _) => rhs.clone(),
            (_, None) => self.clone(),
            (Some((min1, max1)), Some((min2, max2))) => BoundingBox {
                extremities: Some((
                    Vect::new(
                        f64::min(min1.x, min2.x),
                        f64::min(min1.y, min2.y),
                        f64::min(min1.z, min2.z),
                    ),
                    Vect::new(
                        f64::max(max1.x, max2.x),
                        f64::max(max1.y, max2.y),
                        f64::max(max1.z, max2.z),
                    ),
                )),
            },
        }
    }
}

impl Mul for &BoundingBox {
    type Output = BoundingBox;

    /// Returns the intersection of the two bounding boxes
    fn mul(self, rhs: Self) -> BoundingBox {
        match (self.extremities, rhs.extremities) {
            (None, _) | (_, None) => BoundingBox::new(),
            (Some((min1, max1)), Some((min2, max2))) => {
                let min = Vect::new(
                    f64::max(min1.x, min2.x),
                    f64::max(min1.y, min2.y),
                    f64::max(min1.z, min2.z),
                );
                let max = Vect::new(
                    f64::min(max1.x, max2.x),
                    f64::min(max1.y, max2.y),
                    f64::min(max1.z, max2.z),
                );

                if min.x > max.x || min.y > max.y || min.z > max.z {
                    BoundingBox::new()
                } else {
                    BoundingBox::new_from_extremities(min, max)
                }
            }
        }
    }
}
