#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn black() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn white() -> Self {
        Self::new(1., 1., 1.)
    }

    pub fn red() -> Self {
        Self::new(1., 0., 0.)
    }

    pub fn green() -> Self {
        Self::new(0., 1., 0.)
    }

    pub fn blue() -> Self {
        Self::new(0., 0., 1.)
    }
}
