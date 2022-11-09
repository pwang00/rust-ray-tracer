use crate::ray::*;
use crate::utilities::Utils;
use crate::vector::*;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: PointR3,
    lower_left_corner: PointR3,
    horizontal: VecR3,
    vertical: VecR3,
    u: VecR3,
    v: VecR3,
    lens_radius: f64,
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: VecR3 = self.lens_radius * VecR3::random_in_unit_disk();
        let offset: VecR3 = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
    pub fn new(
        look_from: PointR3,
        look_at: PointR3,
        vup: VecR3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = Utils::degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross_product(w).normalize();
        let v = w.cross_product(u);

        let origin: VecR3 = look_from;
        let horizontal: VecR3 = focus_dist * viewport_width * u;
        let vertical: VecR3 = focus_dist * viewport_height * v;
        let lower_left_corner: VecR3 = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }
}
