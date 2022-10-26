use std::{vec, iter::Sum};


pub struct Sphere{
    center : (f64, f64, f64),
    radius : f64,
    color : (u8, u8, u8)
}

pub struct VecR3{
    x: f64,
    y: f64,
    z: f64,
}

// Vector operations
pub trait VecOps{
    fn norm(&self) -> f64;
    fn dot_product(&self, other: &VecR3) -> f64;
    fn add(&self, other: &VecR3) -> VecR3;
    fn cross_product(&self, other: &VecR3) -> VecR3;
}

impl VecOps for VecR3{
    fn norm(&self) -> f64{
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn dot_product(&self, other: &VecR3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn add(&self, other: &VecR3) -> VecR3{
        VecR3{
            x: self.x + other.x,
            y: self.y + other.y, 
            z: self.z + other.z
        }
    }

    fn cross_product(&self, other: &VecR3) -> VecR3 {
        
    }

}