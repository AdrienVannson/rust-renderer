use crate::{BoundingBox, Collision, Ray, Shape};

/// Intersection of two shapes
#[derive(Clone, Debug)]
pub struct Intersection {
    shapes: [Box<dyn Shape>; 2],
}

impl Intersection {
    pub fn new(shape1: Box<dyn Shape>, shape2: Box<dyn Shape>) -> Box<Intersection> {
        Box::new(Self {
            shapes: [shape1, shape2],
        })
    }
}

impl Shape for Intersection {
    fn bounding_box(&self) -> BoundingBox {
        &self.shapes[0].bounding_box() * &self.shapes[1].bounding_box()
    }

    fn collision_date(&self, ray: Ray) -> Option<f64> {
        if let Some(col) = self.collision(ray) {
            Some(col.date)
        } else {
            None
        }
    }

    fn collision(&self, mut ray: Ray) -> Option<Collision> {
        let mut is_inside = [
            self.shapes[0].ray_starts_inside(ray),
            self.shapes[1].ray_starts_inside(ray),
        ];

        /*if self.shapes[0].ray_starts_inside(ray) {
            println!("{:?}", ray);
            println!("{}", (ray.pos - crate::vect::Vect::new(1., 1., -1.)).norm());
            assert!(false);
        }*/

        let mut time_spent = 0.;

        loop {
            let (date, i) = match (
                self.shapes[0].collision_date(ray),
                self.shapes[1].collision_date(ray),
            ) {
                (None, None) => return None,
                (Some(date), None) => (date, 0),
                (None, Some(date)) => (date, 1),
                (Some(date1), Some(date2)) => {
                    if date1 < date2 {
                        (date1, 0)
                    } else {
                        (date2, 1)
                    }
                }
            };

            // TODO we leave the object

            // The ray enters the object
            if !is_inside[i] && is_inside[1 - i] {
                let col = self.shapes[i].collision(ray).unwrap();

                return Some(Collision {
                    date: col.date + time_spent,
                    pos: col.pos,
                    normal: col.normal,
                });
            }

            ray.move_by(date + 1e-6);
            time_spent += date + 1e-6;
            is_inside[i] = !is_inside[i];
        }
    }
}
