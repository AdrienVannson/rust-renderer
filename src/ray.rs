use crate::vect::Vect;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub pos: Vect,
    pub dir: Vect, // This vector should be normalized
}

impl Ray {
    pub fn new(pos: Vect, mut dir: Vect) -> Self {
        dir.normalize();
        Ray { pos, dir }
    }

    pub fn pos(&self) -> Vect {
        self.pos
    }

    pub fn dir(&self) -> Vect {
        self.dir
    }

    /// Move the position of the ray in the direction pointed by the ray
    pub fn move_by(&mut self, dist: f64) {
        self.pos += dist * self.dir;
    }

    /// Returns the point on the ray at a distance t of the origin of the ray
    pub fn pos_in(&self, t: f64) -> Vect {
        self.pos + t * self.dir
    }
}
