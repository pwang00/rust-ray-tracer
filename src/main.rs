use ray_tracer::camera::*;
use ray_tracer::color::*;
use ray_tracer::hittable_list::*;
use ray_tracer::ray::*;
use ray_tracer::sphere::Sphere;
use ray_tracer::utilities::*;
use ray_tracer::vector::*;
use std::rc::Rc;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: u32 = 400;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_DEPTH: u32 = 100;

fn render_image_ppm(width: u32, height: u32) {
    let mut world: HittableList = HittableList {
        objects: Vec::new(),
    };
    let (c1, c2) = (
        PointR3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        PointR3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
    );

    world.add_obj(Rc::new(Sphere {
        center: c1,
        radius: 0.5,
    }));
    world.add_obj(Rc::new(Sphere {
        center: c2,
        radius: 100.0,
    }));

    let viewport_height: f64 = 2.0;
    let focal_length: f64 = 1.0;
    let cam: Camera = Camera::init(ASPECT_RATIO, viewport_height, focal_length);

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
