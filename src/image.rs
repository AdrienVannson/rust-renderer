use crate::Color;
use std::fs::create_dir_all;

pub struct Image {
    pixels: Vec<Vec<Color>>,
}

impl Image {
    /// Creates a new black image
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            pixels: vec![vec![Color::black(); height]; width],
        }
    }

    pub fn width(&self) -> usize {
        self.pixels.len()
    }

    pub fn height(&self) -> usize {
        self.pixels[0].len()
    }

    /// Returns the color of a pixel
    pub fn pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[x][y]
    }

    /// Sets the color of a pixel
    pub fn set_pixel(&mut self, x: usize, y: usize, new_color: Color) {
        self.pixels[x][y] = new_color;
    }

    /// Exports the image to a given file
    pub fn export(&self, filename: &str) {
        // Create the output directory
        create_dir_all("output").expect("Can't create output folder");

        let mut output: image::RgbImage =
            image::ImageBuffer::new(self.width() as u32, self.height() as u32);

        for x in 0..self.width() {
            for y in 0..self.height() {
                output.put_pixel(
                    x as u32,
                    (self.height() - y - 1) as u32,
                    image::Rgb([
                        (255. * self.pixels[x][y].red) as u8,
                        (255. * self.pixels[x][y].green) as u8,
                        (255. * self.pixels[x][y].blue) as u8,
                    ]),
                );
            }
        }

        output
            .save("output/".to_owned() + filename)
            .expect("Could not save the image");
    }
}
