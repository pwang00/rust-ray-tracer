use crate::color::SAMPLES_PER_PIXEL;
use crate::ray::*;
use crate::vector::*;

pub struct RenderParams {
    pub aspect_ratio: f64,
    pub height: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub aperture: f64,
    pub focus_dist: f64,
    pub look_from: VecR3,
    pub look_at: VecR3,
    pub vup: VecR3,
    pub samples_per_pixel: u32,
}

pub const DEFAULT_PARAMS: RenderParams = RenderParams {
    aspect_ratio: 3.0 / 2.0,
    height: 200,
    max_depth: 50,
    vfov: 20.0,
    aperture: 0.1,
    focus_dist: 10.0,
    look_from: PointR3::new(13.0, 2.0, 3.0),
    look_at: PointR3::new(0.0, 0.0, 0.0),
    vup: VecR3::new(0.0, 1.0, 0.0),
    samples_per_pixel: 100,
};
