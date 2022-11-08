use ray_tracer::camera::*;
use ray_tracer::color::*;
use ray_tracer::hittable_list::*;
use ray_tracer::render_params::RenderParams;
use ray_tracer::scene::*;
use ray_tracer::ray::*;
use ray_tracer::utilities::*;
use ray_tracer::vector::*;
use ray_tracer::render_params::DEFAULT_PARAMS;

fn render_image_ppm(params: &RenderParams) {
    let mut world: HittableList = random_scene();

    let cam: Camera = Camera::new(
        params.look_from,
        params.look_at,
        params.vup,
        params.vfov,
        params.aspect_ratio,
        params.aperture,
        params.focus_dist,
    );

    let height = params.height;
    let width = (params.aspect_ratio * params.height as f64) as u32;

    println!("P3\n{} {}\n255", width, height);
    for j in (-1..height as i32 - 1).rev() {
        for i in 0..width {
            let mut vec: VecR3 = VecR3::zero();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u: f64 = (i as f64 + random_double_01()) / (width - 1) as f64;
                let v: f64 = (j as f64 + random_double_01()) / (height - 1) as f64;
                let r: Ray = cam.get_ray(u, v);
                vec += vec_from_ray(r, &mut world, params.max_depth);
            }
            write_pixel(vec.to_pixel(SCALE));
        }
    }
}

fn main() {
    render_image_ppm(&DEFAULT_PARAMS);
}
