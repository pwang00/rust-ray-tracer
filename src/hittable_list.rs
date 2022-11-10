use crate::hittable::*;
use crate::ray::*;
use std::vec::Vec;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn clear_list(&mut self) {
        self.objects.clear()
    }
    pub fn add_obj(&mut self, obj: Box<dyn Hittable + Send + Sync>) {
        self.objects.push(obj)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut any_hits: bool = false;
        let mut curr_closest: f64 = t_max;
        let mut rec = rec;

        for object in &self.objects {
            if object.hit(r, t_min, curr_closest, &mut rec) {
                any_hits = true;
                curr_closest = rec.t;
            }
        }

        any_hits
    }
}
