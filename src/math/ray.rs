use super::Vec3;

use crate::types::Float;

const EPSILON: Float = 1e-4;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub t_min: Float,
    pub t_max: Float,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction, t_min: EPSILON, t_max: Float::INFINITY }
    }

    pub fn at(&self, t: Float) -> Vec3 {
        self.origin + self.direction * t
    }
}