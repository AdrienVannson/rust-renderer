use crate::vect::Vect;

pub struct Camera {
    // Position of the focus point
    pub pos: Vect,

    // Direction where the camera looks. The norm of the vector is the focal
    // distance of the camera.
    pub dir: Vect,

    // Dimensions of the image
    pub width: f64,
    pub height: f64,
}

impl Camera {
    /// Returns the final image
    pub fn render(&self) -> Vec<Vec<(i8, i8, i8)>> {
        Vec::new()
    }
}
