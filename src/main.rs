use ray_tracer::image_formats::*;
use ray_tracer::render::*;
use ray_tracer::render_params::DEFAULT_PARAMS;

use std::time::{Duration, Instant};

fn compare_render_times() -> (Duration, Duration) {
    let start = Instant::now();
    render_image_ppm(&DEFAULT_PARAMS);
    let d1 = start.elapsed();

    let start = Instant::now();
    parallelized_render_ppm(&DEFAULT_PARAMS);
    let d2 = start.elapsed();

    (d1, d2)
}

fn main() {
    let filename: &str = "test_rayon.ppm";
    println!("Starting benchmark...");

    let (d1, d2) = compare_render_times();

    println!("Unparallelized: {:?} vs Parallelized: {:?}", d1, d2);
    println!("Starting render...");

    let image = parallelized_render_ppm(&DEFAULT_PARAMS);

    println!("Starting write...");
    if let Err(_) = write_ppm_to_file(&image, filename) {
        println!("Error occurred while writing ppm");
    }
}
