mod camera;
mod collision;
mod color;
mod primitive;
mod primitives;
mod ray;
mod renderer;
mod scene;
mod shape;
mod shapes;
mod vect;

use crate::camera::Camera;
use crate::scene::Scene;
use crate::shapes::sphere::Sphere;
use crate::vect::Vect;

fn main() {
    let width = 640;
    let height = 360;

    let camera = {
        let pos = Vect::new(30., 0., 6.);
        let dir = -2.5 * pos.normalized();
        Camera {
            pos,
            dir,
            width,
            height,
        }
    };
    let mut scene = Scene::new(camera);

    let sphere = Sphere::new(Vect::new(0., 0., 0.), 1.);
    scene.add_object(Box::new(sphere));

    // Render
    let img = scene.camera.render(&scene);

    // Write the output image
    let mut output: image::RgbImage = image::ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            output.put_pixel(
                x,
                y,
                image::Rgb([
                    img[x as usize][y as usize].0,
                    img[x as usize][y as usize].1,
                    img[x as usize][y as usize].2,
                ]),
            );
        }
    }

    output.save("output.png").expect("Could not save the image");
}
