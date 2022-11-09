
use rand::Rng;
use std::f64::consts::PI;
pub struct Utils;

impl Utils {
    #[inline(always)]
    pub fn degrees_to_radians(deg: f64) -> f64 {
        return deg * PI / 180.0;
    }

    #[inline(always)]
    pub fn random_double_01() -> f64 {
        rand::thread_rng().gen::<f64>()
    }

    #[inline(always)]
    pub fn random_double(min: f64, max: f64) -> f64 {
        min + (max - min) * rand::thread_rng().gen::<f64>()
    }
}

