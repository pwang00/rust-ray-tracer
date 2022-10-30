use crate::ray::*;
use crate::vector::*;

pub struct HitRecord{
    pub p: PointR3,
    pub nv: VecR3,
    pub t: f64
}
pub trait Hittable{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}