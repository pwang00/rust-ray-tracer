use crate::color::*;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utilities::Utils;
use crate::vector::*;

pub struct Lambertian {
    pub albedo: Color,
}
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

pub struct Dielectric {
    pub ir: f64,
}

pub trait Material {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

impl Lambertian {
    pub const fn default() -> Self {
        Lambertian {
            albedo: VecR3::zero(),
        }
    }

    pub const fn new(c: Color) -> Self {
        Lambertian { albedo: c }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir: VecR3 = rec.nv + VecR3::random_unit_vector();

        if scatter_dir.is_near_zero() {
            scatter_dir = rec.nv
        }

        *scattered = Ray {
            origin: rec.p,
            direction: scatter_dir,
        };
        *attenuation = self.albedo;
        true
    }
}

impl Metal {
    pub const fn default() -> Self {
        Metal {
            albedo: VecR3::zero(),
            fuzz: 0.0,
        }
    }

    pub const fn new(c: Color, f: f64) -> Self {
        Metal { albedo: c, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected: VecR3 = VecR3::reflect(r_in.direction.normalize(), rec.nv);
        *scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * VecR3::random_in_unit_sphere(),
        );
        *attenuation = self.albedo;
        scattered.direction.dot_product(rec.nv) > 0.0
    }
}

impl Dielectric {
    pub const fn default() -> Self {
        Dielectric { ir: 1.0 }
    }

    pub const fn new(irf: f64) -> Self {
        Dielectric { ir: irf }
    }

    pub fn reflectance(cos: f64, ri: f64) -> f64 {
        let r0 = ((1.0 - ri) / (1.0 + ri)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = VecR3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.normalize();
        let cos_t = f64::min(-unit_direction.dot_product(rec.nv), 1.0);
        let sin_t = 1.0 - cos_t.powi(2);

        let no_refract = refraction_ratio * sin_t > 1.0;

        let direction: VecR3 = if no_refract
            || Dielectric::reflectance(cos_t, refraction_ratio) > Utils::random_double_01()
        {
            VecR3::reflect(unit_direction, rec.nv)
        } else {
            VecR3::refract(unit_direction, rec.nv, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);

        true
    }
}
