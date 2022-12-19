fn main() {
    let width = 640;
    let height = 360;

    let mut output: image::RgbImage = image::ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            output.put_pixel(x, y, image::Rgb([255, 0, 0]));
        }
    }

    output.save("output.png").expect("Could not save the image");
}
