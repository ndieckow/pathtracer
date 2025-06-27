pub mod math;
pub mod types;

use math::Vec3;
use image::{RgbImage, Rgb};

const WIDTH: usize = 64;
const HEIGHT: usize = 64;

fn render_to_image(&mut framebuffer: Vec<Vec3>) -> RgbImage {
    
}

fn main() {    
    let mut framebuffer: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];

}
