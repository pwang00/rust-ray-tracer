use rand::Rng;

use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vector::*;
use crate::{hittable::HitRecord};
use crate::color::*;

pub const INF: f64 = std::f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;
const TOLERANCE: f64 = 0.0005;

#[inline(always)]
pub fn degrees_to_radians(deg: f64) -> f64 {
    return deg * PI / 180.0;
}

#[inline(always)]
pub fn random_double_01() -> f64 {
    rand::thread_rng().gen::<f64>()
}

#[inline(always)]
pub fn random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::thread_rng().gen::<f64>()
}

pub fn vec_from_ray(r: Ray, world: &dyn Hittable, depth: u32) -> VecR3 {
    let mut rec: HitRecord = HitRecord::new();

    if depth <= 0 {
        return Color::zero();
    }

    if world.hit(r, TOLERANCE, INF, &mut rec) {
        let mut scattered: Ray = Ray::zero();
        let mut attenuation: Color = Color::zero();
        
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered){
            return attenuation * vec_from_ray(scattered, world, depth - 1)
        }
        return Color::zero()
        
    }

    let unit_vec = r.direction.normalize();
    let t = 0.5 * (unit_vec.y + 1.0);

    (1.0 - t)
        * VecR3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + t * VecR3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
}
