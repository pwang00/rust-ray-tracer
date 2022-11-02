use std::{fmt, ops};
use crate::color::*;
use crate::utilities::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct VecR3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Vector operations
impl fmt::Display for VecR3{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Neg for VecR3{
    type Output = VecR3;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl ops::Add<VecR3> for VecR3{
    type Output = VecR3;

    fn add(self, rhs: VecR3) -> Self::Output {
        VecR3{
            x: self.x + rhs.x,
            y: self.y + rhs.y, 
            z: self.z + rhs.z
        }   
    }
}

impl ops::Sub<VecR3> for VecR3{
    type Output = VecR3;

    fn sub(self, rhs: VecR3) -> Self::Output {
        VecR3{
            x: self.x - rhs.x,
            y: self.y - rhs.y, 
            z: self.z - rhs.z
        }   
    }
}

impl ops::Mul<f64> for VecR3{
    type Output = VecR3;

    fn mul(self, rhs: f64) -> Self::Output {
        VecR3{
            x: self.x * rhs,
            y: self.y * rhs, 
            z: self.z * rhs
        }   
    }
}

impl ops::Mul<VecR3> for f64{
    type Output = VecR3;

    fn mul(self, rhs: VecR3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for VecR3{
    type Output = VecR3;

    fn div(self, rhs: f64) -> Self::Output {
        VecR3{
            x: self.x / rhs,
            y: self.y / rhs, 
            z: self.z / rhs
        }   
    }
}

impl ops::AddAssign<VecR3> for VecR3{
    fn add_assign(&mut self, rhs: VecR3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl VecR3{
    pub fn zero() -> Self{
        VecR3{x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn norm(&self) -> f64{
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot_product(self, other: VecR3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(self, other: VecR3) -> VecR3 {
        VecR3{
            x: (self.y * other.z - self.z * other.y),
            y: -(self.x * other.z - self.z * other.y),
            z: (self.x * other.y - self.y * other.x)
        }
    }

    pub fn normalize(&self) -> VecR3{
        let norm: f64 = self.norm();
        VecR3{
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm
        }
    }

    pub fn to_pixel(&self, scale: f64) -> Pixel {
        Pixel { 
            r: (256.0 * clamp((self.x * scale).powf(1.0/2.5), 0.0, 0.999)) as u8,
            g: (256.0 * clamp((self.y * scale).powf(1.0/2.5), 0.0, 0.999)) as u8, 
            b: (256.0 * clamp((self.z * scale).powf(1.0/2.5), 0.0, 0.999)) as u8 
        }
    }

    pub fn random_vec_01() -> VecR3{
        VecR3 { 
            x: random_double_01(),
            y: random_double_01(), 
            z: random_double_01() 
        }
    }

    pub fn random_vec(min: f64, max: f64) -> VecR3{
        VecR3 { 
            x: random_double(min, max),
            y: random_double(min, max), 
            z: random_double(min, max) 
        }
    }
}