use std::ops::{Add, Div, Mul, Neg, Sub};

use rand_distr::{Distribution, Normal, NormalError};
//use rand;

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

    pub fn random() -> Result<Self, NormalError> {
        let mut rng = rand::rng();
        let normal = Normal::new(0.0, 1.0)?;
        Ok(Self {
            x: normal.sample(&mut rng),
            y: normal.sample(&mut rng),
            z: normal.sample(&mut rng),
        }
        .normalize())
    }

    pub fn dot(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm_sq(&self) -> Float {
        self.dot(self)
    }

    pub fn norm(&self) -> Float {
        self.norm_sq().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/*
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
    */

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

impl Mul<Vec3> for Float {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self * vec.x,
            y: self * vec.y,
            z: self * vec.z,
        }
    }
}

impl Div<Float> for Vec3 {
    type Output = Self;

    fn div(self, a: Float) -> Self {
        Self {
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
        }
    }
}
