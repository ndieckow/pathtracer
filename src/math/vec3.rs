use std::ops::{Add, Mul};

use crate::types::Float;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vec3) -> Float {
        self.x * other.x + self.y * other.y * self.z * other.z
    }

    pub fn norm_sq(&self) -> Float {
        self.dot(self)
    }

    pub fn norm(&self) -> Float {
        self.norm_sq().sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Mul<Float> for Vec3 {
    type Output = Self;

    fn mul(self, a: Float) -> Self {
        Self {
            x: a * self.x,
            y: a * self.y,
            z: a * self.z,
        }
    }
}