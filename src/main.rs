use std::rc::Rc;

use ray_tracer::hittable::*;
use ray_tracer::hittable_list::*;
use ray_tracer::sphere::Sphere;
use ray_tracer::vector::*;
use ray_tracer::ray::*;
use ray_tracer::color::*;
use ray_tracer::utilities::*;

const ASPECT_RATIO: f64 = 16.0/9.0;
const WIDTH: u32 = 400;
const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

fn ray_to_pixel(r: Ray, world: &dyn Hittable) -> Pixel{
    let mut rec: HitRecord = DEFAULT_HIT_RECORD;

    if world.hit(r, 0.0, INF, &mut rec){
        return (0.5 * (rec.nv + VecR3{x: 1.0, y: 1.0, z: 1.0})).to_pixel()
    }

    let unit_vec = r.direction.normalize();
    let t = 0.5 * (unit_vec.y + 1.0);
    let p = (1.0 - t) * VecR3{x: 1.0, y: 1.0, z: 1.0} 
                        + t * VecR3{x: 0.5, y: 0.7, z: 1.0};

    p.to_pixel()
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
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: PointR3 = PointR3{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = VecR3{x: viewport_width as f64, y: 0.0, z: 0.0};
    let vertical = VecR3{x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 
                                        - VecR3{x: 0.0, y: 0.0, z: focal_length};
    
    println!("P3\n{} {}\n255", width, height);
    for j in (-1..height as i32 - 1).rev(){

        for i in 0..width{
            let u: f64 = i as f64 / width as f64;
            let v: f64 = j as f64 / height as f64;
            let r: Ray = Ray{
                            origin: origin, 
                            direction: lower_left_corner + u * horizontal + v * vertical - origin
                        };

            let pix: Pixel = ray_to_pixel(r, &world);
            write_pixel(pix);
        }
    }
}

fn main() {
    render_image_ppm(WIDTH, HEIGHT);
}

