use crate::math::{Vec3, Ray};
use crate::types::Float;

pub struct Camera {
    pub position: Vec3,
    pub forward: Vec3, // Facing direction
    pub fov: Float, // Field of view in degrees
    pub focal_length: Float,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, fov: Float, focal_length: Float, aspect_ratio: Float) -> Self {
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let forward = (look_at - position).normalize();
        let right = -forward.cross(&world_up).normalize(); // negate to un-mirror axes
        let up = right.cross(&forward);

        let theta = fov.to_radians();
        let h = (theta / 2.0).tan(); // half the height of the image plane at a unit distance (z = 1) from the camera

        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let horizontal = right * viewport_width * focal_length;
        let vertical = up * viewport_height * focal_length;

        let lower_left_corner = position + forward * focal_length - horizontal * 0.5 - vertical * 0.5;

        Self {
            position,
            forward,
            fov,
            focal_length,
            lower_left_corner,
            horizontal,
            vertical
        }
    }

    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        let direction = self.lower_left_corner + self.horizontal * s + self.vertical * t - self.position;
        Ray::new(self.position, direction)
    }
}
