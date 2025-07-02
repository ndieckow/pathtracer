use crate::math::{Ray, Vec3};
use crate::types::Float;

pub struct Sphere {
    pub center: Vec3,
    pub radius: Float,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float) -> Self {
        Self { center, radius }
    }

    pub fn ray_intersection(&self, ray: &Ray) -> Option<Float> {
        let oc = ray.origin - self.center;

        let a = ray.direction.norm_sq();
        let b = 2.0 * ray.direction.dot(&oc);
        let c = oc.norm_sq() - self.radius * self.radius;
        
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrt_disc = discriminant.sqrt();
            let t1 = (-b - sqrt_disc) / (2.0 * a);
            let t2 = (-b + sqrt_disc) / (2.0 * a);
            let t = if t1 >= ray.t_min && t1 <= ray.t_max {
                t1
            } else if t2 >= ray.t_min && t2 <= ray.t_max {
                t2
            } else {
                return None;
            };

            Some(t)
        }
    }
}