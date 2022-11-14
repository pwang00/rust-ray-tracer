use crate::render::*;
use crate::render_params::*;
use std::time::{Duration, Instant};

pub fn compare_render_times() -> (Duration, Duration) {
    println!("Starting unparallelized render...");
    let start = Instant::now();
    unparallelized_render_image(&DEFAULT_PARAMS);
    let d1 = start.elapsed();

    println!("Finished unparallelized render.  Starting parallelized render...");

    let start = Instant::now();
    parallelized_render_image(&DEFAULT_PARAMS);
    let d2 = start.elapsed();

    (d1, d2)
}
