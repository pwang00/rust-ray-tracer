use core::panic;
use std::{fmt, ops};
use crate::color::*;

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

impl VecR3{
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

    pub fn to_pixel(&self) -> Pixel {
        if self.x > 1.0 || self.y > 1.0 || self.z > 1.0{
            panic!("Vector components are not normalized!  Failure with vector {}", self)
        }

        Pixel { 
            r: (self.x * 255.9) as u8, 
            g: (self.y * 255.9) as u8, 
            b: (self.z * 255.9) as u8 
        }
    }

}