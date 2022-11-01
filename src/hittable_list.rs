use std::rc::*;
use std::vec::Vec;
use crate::utilities::*;
use crate::hittable::*;
use crate::ray::*;

pub struct HittableList{
    pub objects: Vec<Rc<dyn Hittable>>
}

pub trait HittableListOps{
    fn clear_list(&mut self);
    fn add_obj(&mut self, obj: Rc<dyn Hittable>);
}

impl HittableListOps for HittableList{
    fn clear_list(&mut self) {
        self.objects.clear()
    }
    fn add_obj(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj)
    }
}

impl Hittable for HittableList{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = DEFAULT_HIT_RECORD;
        let mut any_hits: bool = false;
        let mut curr_closest: f64 = t_max;

        for object in &self.objects{
            if object.hit(r, t_min, curr_closest, &mut temp_rec){
                any_hits = true;
                curr_closest = temp_rec.t;

                // Copy fields over manually since borrow checker L
                rec.front_face = temp_rec.front_face;
                rec.p = temp_rec.p;
                rec.nv = temp_rec.nv;
                rec.t = temp_rec.t;
            }
        }
        
        any_hits
    }
}