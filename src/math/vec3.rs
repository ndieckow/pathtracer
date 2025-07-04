use std::{
    f32::consts::PI,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

use rand::Rng;
use rand_distr::{Distribution, Normal, NormalError};

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

    pub fn rand_disk() -> Self {
        let mut rng = rand::rng();
        let u_offset = rng.random_range(-1.0..1.0);
        let v_offset = rng.random_range(-1.0..1.0);
        if u_offset == 0.0 && v_offset == 0.0 {
            return Self::zeros();
        }

        let (theta, r) = if Float::abs(u_offset) > Float::abs(v_offset) {
            (PI / 4.0 * (v_offset / u_offset), u_offset)
        } else {
            (PI / 2.0 - PI / 4.0 * (u_offset / v_offset), v_offset)
        };

        r * Self::new(theta.cos(), theta.sin(), 0.0)
    }

    pub fn rand_hemisphere() -> Self {
        let mut rng = rand::rng();
        let z = rng.random_range(0.0..1.0);
        let r = ((1.0 - z * z) as Float).sqrt();
        let phi = 2.0 * PI * rng.random_range(0.0..1.0);

        Self {
            x: r * phi.cos(),
            y: r * phi.sin(),
            z,
        }
    }

    pub fn rand_hemisphere_cosine() -> Self {
        let d = Self::rand_disk();
        let z = Float::max(0.0, 1.0 - d.x * d.x - d.y * d.y).sqrt();
        Self { x: d.x, y: d.y, z }
    }

    pub fn zeros() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn extend_to_onb(&self) -> (Self, Self) {
        let a = if self.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let tangent = self.cross(&a).normalize();
        let bitangent = self.cross(&tangent);
        (tangent, bitangent)
    }

    pub fn lerp(a: Self, b: Self, t: Float) -> Self {
        a * (1.0 - t) + b * t
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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) -> () {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
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

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
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
