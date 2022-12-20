use crate::vect::Vect;

#[derive(Copy, Clone)]
pub struct Ray {
    pos: Vect,
    dir: Vect, // This vector should be normalized
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
}
