use crate::ray::*;
use crate::vector::*;
use std::fmt;

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
    pub tolerance: f64,
}

impl fmt::Display for RenderParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RenderParams {{").unwrap();
        writeln!(f, "    height: {}", self.height).unwrap();
        writeln!(f, "    aspect_ratio: {}", self.aspect_ratio).unwrap();
        writeln!(f, "    max_depth: {}", self.height).unwrap();
        writeln!(f, "    vfov: {}", self.vfov).unwrap();
        writeln!(f, "    aperture: {}", self.focus_dist).unwrap();
        writeln!(f, "    look_from: {}", self.look_from).unwrap();
        writeln!(f, "    look_at: {}", self.look_at).unwrap();
        writeln!(f, "    vup: {}", self.vup).unwrap();
        writeln!(f, "    samples_per_pixel: {}", self.samples_per_pixel).unwrap();
        writeln!(f, "    tolerance: {}", self.tolerance).unwrap();
        write!(f, "}}")
    }
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
    tolerance: 0.001,
};
