mod camera;
mod collision;
mod color;
mod ray;
mod scene;
mod shape;
mod sphere;
mod vect;
mod renderer;
mod primitive;

use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::vect::Vect;

fn main() {
    let width = 640;
    let height = 360;

    let mut output: image::RgbImage = image::ImageBuffer::new(width, height);

    let mut scene = Scene::new();

    let sphere = Sphere::new(Vect::new(0., 0., 0.), 1.);
    scene.add_object(Box::new(sphere));

    for x in 0..width {
        for y in 0..height {
            output.put_pixel(x, y, image::Rgb([255, 0, 0]));
        }
    }

    output.save("output.png").expect("Could not save the image");
}
