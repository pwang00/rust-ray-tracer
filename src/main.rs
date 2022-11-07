use ray_tracer::camera::*;
use ray_tracer::color::*;
use ray_tracer::hittable_list::*;
use ray_tracer::scene::*;
use ray_tracer::ray::*;
use ray_tracer::utilities::*;
use ray_tracer::vector::*;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const WIDTH: u32 = 1200;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_DEPTH: u32 = 50;
const VFOV: f64 = 20.0;

fn render_image_ppm(width: u32, height: u32) {
    let mut world: HittableList = random_scene();
    let look_from = PointR3::new(13.0, 2.0, 3.0);
    let look_at = PointR3::new(0.0, 0.0, 0.0);
    let vup = VecR3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam: Camera = Camera::new(
        look_from,
        look_at,
        vup,
        VFOV,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    println!("P3\n{} {}\n255", width, height);
    for j in (-1..height as i32 - 1).rev() {
        for i in 0..width {
            let mut vec: VecR3 = VecR3::zero();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u: f64 = (i as f64 + random_double_01()) / (WIDTH - 1) as f64;
                let v: f64 = (j as f64 + random_double_01()) / (HEIGHT - 1) as f64;
                let r: Ray = cam.get_ray(u, v);
                vec += vec_from_ray(r, &mut world, MAX_DEPTH);
            }
            write_pixel(vec.to_pixel(SCALE));
        }
    }
}

fn main() {
    render_image_ppm(WIDTH, HEIGHT);
}
