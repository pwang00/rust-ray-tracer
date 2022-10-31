use crate::ray::*;
use crate::vector::*;

#[derive(Clone, Copy)]
pub struct HitRecord{
    pub p: PointR3,
    pub nv: VecR3,
    pub t: f64,
    pub front_face: bool,
}
pub trait Hittable{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub trait DetermineNormal{
    fn set_face_normal(&mut self, r: Ray, outward_normal: VecR3);
}

impl DetermineNormal for HitRecord{
    fn set_face_normal(&mut self, r: Ray, outward_normal: VecR3) {
        self.front_face = r.direction.dot_product(outward_normal) < 0.0;
        self.nv = if self.front_face {outward_normal} else {-outward_normal}
    }   
}