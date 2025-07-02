use super::HitRecord;
use crate::math::{Ray, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let scatter_dir = hit.normal + Vec3::random().unwrap();
        let out_ray = Ray::new(hit.point, scatter_dir);
        Some((out_ray, self.albedo))
    }
}
