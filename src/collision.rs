use crate::vect::Vect;

pub struct Collision {
    /// The date at which the collision occurs
    pub date: f64,

    /// The position of the collision
    pub pos: Vect,

    /// The normal of the object at the position of collision
    pub normal: Vect,
}
