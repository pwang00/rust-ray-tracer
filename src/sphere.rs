use crate::hittable::*;
use crate::ray::*;
use crate::vector::*;
use crate::material::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    pub center: PointR3,
    pub radius: f64,
    pub mat: Rc<dyn Material>
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: VecR3 = r.origin - self.center;
        let a: f64 = r.direction.norm().powi(2);
        let half_b = oc.dot_product(r.direction);
        let c = oc.norm().powi(2) - self.radius.powi(2);

        // Quadratic discriminant
        let qd = half_b.powi(2) - a * c;
        if qd < 0.0 {
            return false;
        }

        // Roots are (-b +/- sqrt(b^2 - 4ac)) / 2a
        let sqrt_qd = qd.sqrt();
        let mut root = (-half_b - sqrt_qd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrt_qd) / a;

            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.point_at(rec.t);
        rec.nv = (rec.p - self.center) / self.radius;

        let outward_normal: VecR3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();
        return true;
    }
}
