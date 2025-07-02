pub mod math;
pub mod scene;
pub mod types;

use std::error::Error;
use std::sync::Arc;

use image::{Rgb, RgbImage};

use math::{Vec3, Ray};
use scene::{Camera, HitRecord, Object, Sphere};
use scene::material::{Material, Lambertian};
use types::Float;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const SAMPLES_PER_PIXEL: u32 = 50;

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

fn trace_ray(ray: &Ray, objects: &Vec<Box<dyn Object>>, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::zeros();
    }

    let mut closest_hit: Option<HitRecord> = None;
    let mut closest_t: Float = Float::INFINITY;

    for obj in objects.iter() {
        if let Some(hit) = obj.ray_intersection(ray) {
            if hit.t < closest_t {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }
    }

    if let Some(hit) = closest_hit {
        if let Some((out_ray, col)) = hit.material.scatter(ray, &hit) {
            return col * trace_ray(&out_ray, objects, depth - 1);
        } else {
            return Vec3::zeros();
        }
    }

    // Background gradient
    let unit_dir = ray.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    Vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.0, 0.2), t)
}

fn render_scene(objects: &Vec<Box<dyn Object>>, camera: &Camera, framebuffer: &mut Vec<Vec3>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = y * WIDTH + x;
            let ray = camera.get_ray(
                (x as Float) / (WIDTH as Float),
                (y as Float) / (HEIGHT as Float),
                1.0,
            );
            let mut color = Vec3::zeros();
            for _ in 0..SAMPLES_PER_PIXEL {
                color += trace_ray(&ray, objects, 5);
            }
            framebuffer[index] = color / SAMPLES_PER_PIXEL as Float;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut framebuffer: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];

    let objects: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere {
            center: Vec3::new(2.0, 0.0, 2.0),
            radius: 0.5,
            material: Arc::new(Lambertian { albedo: Vec3::new(1.0, 0.0, 0.0) }),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 1.0, 2.0),
            radius: 1.0,
            material: Arc::new(Lambertian { albedo: Vec3::new(0.0, 1.0, 0.0) }),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 1.0, 4.0),
            radius: 1.5,
            material: Arc::new(Lambertian { albedo: Vec3::new(0.0, 0.0, 1.0) }),
        }),
    ];
    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 90.0);

    render_scene(&objects, &camera, &mut framebuffer);

    let img = render_to_image(&framebuffer);
    img.save("output.png")?;
    Ok(())
}
