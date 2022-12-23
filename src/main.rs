mod camera;
mod color;
mod light;
mod material;
mod primitive;
mod primitives;
mod ray;
mod renderer;
mod renderers;
mod scene;
mod shape;
mod shapes;
mod vect;

use camera::Camera;
use color::Color;
use light::Light;
use primitives::checkerboard::Checkerboard;
use primitives::geometric_primitive::GeometricPrimitive;
use renderers::whitted_ray_tracer::WhittedRayTracer;
use scene::Scene;
use shapes::sphere::Sphere;
use vect::Vect;

fn main() {
    let width = 640;
    let height = 360;

    let camera = {
        let pos = Vect::new(0., -8., 6.);
        let dir = -2.5 * pos.normalized();
        Camera {
            pos,
            dir,
            width,
            height,
        }
    };

    let mut scene = Scene::new(camera);

    scene.add_light(Light {
        pos: Vect::new(0., 0., 100.),
        intensity: 1.,
    });

    let sphere = GeometricPrimitive::new(Box::new(Sphere::new(Vect::new(0., 0., 1.), 1.)));
    scene.add_primitive(Box::new(sphere));

    let checkboard = Checkerboard::new(
        Vect::new(-3., -3., 0.),
        6.,
        6.,
        12,
        12,
        Color::new(0., 1., 1.),
        Color::new(1., 0., 1.),
    );
    scene.add_primitive(Box::new(checkboard));

    // Render
    let renderer = WhittedRayTracer {};

    let img = scene.camera.render(&scene, &renderer);

    // Write the output image
    let mut output: image::RgbImage = image::ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            output.put_pixel(
                x,
                height - y - 1,
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
