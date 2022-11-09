use crate::color::*;
use crate::color::*;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::vector::VecR3;
use crate::vector::*;
use std::f64::INFINITY;
pub type PointR3 = VecR3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: PointR3,
    pub direction: VecR3,
}

impl Ray {
    pub fn new(origin: VecR3, direction: VecR3) -> Self {
        Ray { origin, direction }
    }
    pub fn zero() -> Self {
        Ray {
            origin: PointR3::zero(),
            direction: VecR3::zero(),
        }
    }

    pub fn point_at(&self, t: f64) -> PointR3 {
        self.origin + t * self.direction
    }

    pub fn vec_from_ray(r: Ray, world: &dyn Hittable, depth: u32, tolerance: f64) -> VecR3 {
        let mut rec: HitRecord = HitRecord::new();

        if depth <= 0 {
            return Color::zero();
        }

        if world.hit(r, tolerance, INFINITY, &mut rec) {
            let mut scattered: Ray = Ray::zero();
            let mut attenuation: Color = Color::zero();

            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Ray::vec_from_ray(scattered, world, depth - 1, tolerance);
            }
            return Color::zero();
        }

        let unit_vec = r.direction.normalize();
        let t = 0.5 * (unit_vec.y + 1.0);

        (1.0 - t) * VecR3::new(1.0, 1.0, 1.0) + t * VecR3::new(0.5, 0.7, 1.0)
    }
}
