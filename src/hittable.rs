use crate::material::*;
use crate::ray::*;
use crate::vector::*;

use std::rc::Rc;
pub struct HitRecord {
    pub p: PointR3,
    pub nv: VecR3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}
pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: PointR3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            nv: PointR3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            mat: Rc::new(Lambertian::default()),
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: VecR3) {
        self.front_face = r.direction.dot_product(outward_normal) < 0.0;
        self.nv = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
