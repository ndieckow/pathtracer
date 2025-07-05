use std::sync::Arc;

use super::Material;
use crate::math::{Ray, Vec3};
use crate::types::Float;

pub struct HitRecord {
    pub t: Float,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material + Send + Sync>,
}

pub trait Object {
    fn ray_intersection(&self, ray: &Ray) -> Option<HitRecord>;

    fn material(&self) -> Arc<dyn Material + Send + Sync>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: Float,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Object for Sphere {
    fn ray_intersection(&self, ray: &Ray) -> Option<HitRecord> {
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

            let point = ray.at(t);
            let normal = (point - self.center).normalize();
            Some(HitRecord {
                t,
                point,
                normal,
                material: Arc::clone(&self.material),
            })
        }
    }

    fn material(&self) -> Arc<dyn Material + Send + Sync> {
        Arc::clone(&self.material)
    }
}

pub struct Plane {
    pub center: Vec3,
    pub normal: Vec3,
    pub size: Float, // negative means infinite
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Object for Plane {
    fn ray_intersection(&self, ray: &Ray) -> Option<HitRecord> {
        let d_dot_n = ray.direction.dot(&self.normal);
        let c_minus_o_dot_n = (self.center - ray.origin).dot(&self.normal);
        if d_dot_n.abs() < 0.001 || d_dot_n.signum() != c_minus_o_dot_n.signum() {
            return None;
        }
        let t = c_minus_o_dot_n / d_dot_n;
        let hit_point = ray.at(t);

        if (hit_point - self.center).norm() <= self.size {
            Some(HitRecord {
                t,
                point: hit_point,
                normal: if d_dot_n < 0.0 { self.normal } else { -self.normal },
                material: Arc::clone(&self.material),
            })
        } else {
            None
        }
    }

    fn material(&self) -> Arc<dyn Material + Send + Sync> {
        Arc::clone(&self.material)
    }
}
