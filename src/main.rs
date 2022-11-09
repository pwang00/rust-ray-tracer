use ray_tracer::render_params::DEFAULT_PARAMS;
use ray_tracer::render::render_image_ppm;

fn main() {
    render_image_ppm(&DEFAULT_PARAMS);
}
