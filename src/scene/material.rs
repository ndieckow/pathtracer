use super::HitRecord;
use crate::math::{Ray, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
    fn emitted(&self, ray: &Ray, hit: &HitRecord) -> Vec3 {
        Vec3::zeros()
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let local_dir = Vec3::rand_hemisphere_cosine();
        let (tangent, bitangent) = hit.normal.extend_to_onb();
        let world_dir = tangent * local_dir.x + bitangent * local_dir.y + hit.normal * local_dir.z; // TODO: express as matrix-vector multiply

        let scatter_dir = hit.normal + world_dir;
        let out_ray = Ray::new(hit.point, scatter_dir);
        Some((out_ray, self.albedo))
    }
}

pub struct Emissive {
    pub emitted_color: Vec3,
}

impl Material for Emissive {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }
    fn emitted(&self, ray: &Ray, hit: &HitRecord) -> Vec3 {
        self.emitted_color
    }
}