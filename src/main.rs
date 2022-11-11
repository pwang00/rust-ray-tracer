use ray_tracer::render_params::DEFAULT_PARAMS;
use ray_tracer::{image_formats::write_ppm_to_file, render::*};
use std::time::{Duration, Instant};

fn compare_render_times() -> (Duration, Duration) {
    let start = Instant::now();
    render_image_ppm(&DEFAULT_PARAMS);
    let d1 = start.elapsed();

    println!("Finished unparallelized render.  Starting parallelized...");

    let start = Instant::now();
    parallelized_render_ppm(&DEFAULT_PARAMS);
    let d2 = start.elapsed();

    (d1, d2)
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(8).build_global().unwrap();
    
    println!("Starting benchmark with the following render parameters:\n\n{}\n", &DEFAULT_PARAMS);

    let (d1, d2) = compare_render_times();

    println!("Unparallelized: {:?} vs Parallelized: {:?}", d1, d2);
    
    let filename = "rayon.ppm";
    let image = parallelized_render_ppm(&DEFAULT_PARAMS);
    if let Err(_) = write_ppm_to_file(&image, filename) {
        println!("Error occurred.")
    }
}
