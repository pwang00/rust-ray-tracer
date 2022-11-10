use ray_tracer::render::*;
use ray_tracer::render_params::DEFAULT_PARAMS;
use ray_tracer::image_formats::*;
fn main() {
    //render_image_ppm(&DEFAULT_PARAMS);
    let filename: &str = "test_rayon.ppm";

    println!("Starting render...");
    
    let image = parallelized_render_ppm(&DEFAULT_PARAMS);

    println!("Starting write...");
    if let Err(_) = write_ppm_to_file(&image, filename){
        println!("Error occurred while writing ppm");
    }
}
