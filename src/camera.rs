use crate::ray::*;
use crate::vector::*;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: PointR3,
    lower_left_corner: PointR3,
    horizontal: VecR3,
    vertical: VecR3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
    pub fn init(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let origin: VecR3 = PointR3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let horizontal: VecR3 = VecR3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical: VecR3 = VecR3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let lower_left_corner: VecR3 = origin
            - horizontal / 2.0
            - vertical / 2.0
            - VecR3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}
