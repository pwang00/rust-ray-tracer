use std::rc::Rc;

use ray_tracer::hittable::*;
use ray_tracer::hittable_list::*;
use ray_tracer::sphere::Sphere;
use ray_tracer::vector::*;
use ray_tracer::ray::*;
use ray_tracer::color::*;
use ray_tracer::utilities::*;
use ray_tracer::camera::*;

const ASPECT_RATIO: f64 = 16.0/9.0;
const WIDTH: u32 = 400;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;


fn ray_to_pixel(r: Ray, world: &dyn Hittable) -> VecR3{
    let mut rec: HitRecord = DEFAULT_HIT_RECORD;

    if world.hit(r, 0.0, INF, &mut rec){
        return 0.5 * (rec.nv + VecR3{x: 1.0, y: 1.0, z: 1.0})
    }

    let unit_vec = r.direction.normalize();
    let t = 0.5 * (unit_vec.y + 1.0);
    let p = (1.0 - t) * VecR3{x: 1.0, y: 1.0, z: 1.0} 
                        + t * VecR3{x: 0.5, y: 0.7, z: 1.0};

    p
}

fn render_image_ppm(width: u32, height: u32){
    let mut world: HittableList = HittableList { objects: Vec::new() };
    let (c1, c2) = (
        PointR3{x: 0.0, y: 0.0, z: -1.0}, 
        PointR3{x: 0.0, y: -100.5, z: -1.0}
    );
    
    world.add_obj(Rc::new(Sphere{center: c1, radius: 0.5}));
    world.add_obj(Rc::new(Sphere{center: c2, radius: 100.0}));

    let viewport_height: f64 = 2.0;
    let focal_length: f64 = 1.0;
    let cam: Camera = Camera::init(ASPECT_RATIO, viewport_height, focal_length);

    println!("P3\n{} {}\n255", width, height);
    for j in (-1..height as i32 - 1).rev(){
        for i in 0..width{
            let mut vec: VecR3 = VecR3{x: 0.0, y: 0.0, z: 0.0};
            for _s in 0..SAMPLES_PER_PIXEL{
                let u: f64 = (i as f64 + random_double_01()) / (WIDTH - 1) as f64;
                let v: f64 = (j as f64 + random_double_01()) / (HEIGHT - 1) as f64;
                let r: Ray = cam.get_ray(u, v);
                vec = vec + ray_to_pixel(r, &mut world);
            }
            write_pixel(vec.normalize().to_pixel());
        }
    }
}

fn main() {
    render_image_ppm(WIDTH, HEIGHT);
}

