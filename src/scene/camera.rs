use crate::math::{Vec3, Ray};
use crate::types::Float;

pub struct Camera {
    pub position: Vec3,
    pub forward: Vec3, // Facing direction
    pub up: Vec3,
    pub right: Vec3,
    pub fov: Float, // Field of view in degrees
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, fov: Float) -> Self {
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let forward = (look_at - position).normalize();
        let right = -forward.cross(&world_up).normalize(); // negate to un-mirror axes
        let up = right.cross(&forward);
        Self {
            position,
            forward,
            up,
            right,
            fov,
        }
    }

    pub fn get_ray(&self, s: Float, t: Float, focal_length: Float) -> Ray {
        let theta = self.fov.to_radians();
        let h = (theta / 2.0).tan(); // half the height of the image plane at a unit distance (z = 1) from the camera

        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * 1.0; // 1:1 aspect ratio for now

        let horizontal = self.right * viewport_width * focal_length;
        let vertical = self.up * viewport_height * focal_length;

        let lower_left_corner = self.position + self.forward * focal_length - horizontal * 0.5 - vertical * 0.5;

        let direction = lower_left_corner + horizontal * s + vertical * t - self.position;
        Ray::new(self.position, direction)
    }
}
