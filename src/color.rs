use crate::vector::VecR3;
use std::{fmt, u32};

pub const SAMPLES_PER_PIXEL: u32 = 100;
pub const SCALE: f64 = 1.0 / SAMPLES_PER_PIXEL as f64;
pub type Color = VecR3;

#[derive(Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

#[inline(always)]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

pub fn write_pixel(p: Pixel) {
    println!("{} {} {}", p.r, p.g, p.b);
}
