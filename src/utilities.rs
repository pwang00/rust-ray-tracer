use rand::Rng;

use crate::{hittable::HitRecord, ray::PointR3};

pub const INF: f64 = std::f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;


pub const DEFAULT_HIT_RECORD: HitRecord = HitRecord {
    p: PointR3{x: 0.0, y: 0.0, z: 0.0}, 
    nv: PointR3{x: 0.0, y: 0.0, z: 0.0}, 
    t: 0.0, 
    front_face: false 
};

#[inline(always)]
pub fn degrees_to_radians(deg: f64) -> f64{
    return deg * PI / 180.0
}

#[inline(always)]
pub fn random_double_01() -> f64{
    rand::thread_rng().gen::<f64>()
}

#[inline(always)]
pub fn random_double(min: f64, max: f64) -> f64{
    min + (max - min) * rand::thread_rng().gen::<f64>()
}