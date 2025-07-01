pub mod math;
pub mod types;

use std::error::Error;
use image::{RgbImage, Rgb};

use math::Vec3;

const WIDTH: usize = 64;
const HEIGHT: usize = 64;

fn vec3_to_rgb(color: &Vec3) -> Rgb<u8> {
    let r = (color.x.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8;
    let g = (color.y.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8;
    let b = (color.z.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8;
    Rgb([r, g, b])
}

fn render_to_image(framebuffer: &Vec<Vec3>) -> RgbImage {
    let mut img = RgbImage::new(WIDTH as u32, HEIGHT as u32);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            let color = framebuffer[idx];
            img.put_pixel(x as u32, y as u32, vec3_to_rgb(&color));
        }
    }

    img
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut framebuffer: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
    let img = render_to_image(&framebuffer);
    img.save("output.png")?;
    Ok(())
}
