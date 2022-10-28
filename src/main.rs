use crate::vector::*;
use crate::sphere::*;
use crate::color::*;
use crate::ray::*;

mod vector;
mod sphere;
mod color;
mod ray;

const ASPECT_RATIO: f64 = 16.0/9.0;
const WIDTH: u32 = 400;
const HEIGHT: u32 = WIDTH / ASPECT_RATIO as u32;

fn ray_to_pixel(r: Ray) -> Pixel{

    if hits_sphere(PointR3{x: 0.0, y: 0.0, z: -1.0}, 0.5, r){
        return Pixel{r: 255, g: 0, b: 0}
    }
    // Unit vector pointing in the direction of ray
    let unit_vec = r.direction.normalize();
    let t = 0.5 * unit_vec.y + 1.0;
    let p = (1.0 - t) * VecR3{x: 1.0, y: 1.0, z: 1.0} 
                        + t * VecR3{x: 0.5, y: 0.7, z: 1.0};

    Pixel { r: (p.x * 256.0) as u8, 
            g: (p.y * 256.0) as u8,
            b: (p.z * 256.0) as u8
    }

}

fn hits_sphere(center: PointR3, radius: f64, r: Ray) -> bool{
    let offset: VecR3 = r.origin - center;
    let a: f64 = r.direction.dot_product(r.direction);
    let b: f64 = 2.0 * offset.dot_product(r.direction);
    let c: f64 = offset.dot_product(offset) - radius.powi(2);
    let qd = b.powi(2) - 4.0 * a * c; // Quadratic discriminant = b^2 - 4ac
    return qd > 0.0

}

fn render_image_ppm(width: u32, height: u32){

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: PointR3 = PointR3{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = VecR3{x: viewport_width as f64, y: 0.0, z: 0.0};
    let vertical = VecR3{x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 
                                        - VecR3{x: 0.0, y: 0.0, z: focal_length};
    
    println!("P3\n{} {}\n255", width, height);
    for i in -1..width as i32 - 1{

        for j in 0..height{
            let u: f64 = i as f64 / WIDTH as f64;
            let v: f64 = j as f64 / HEIGHT as f64;
            let r: Ray = Ray{
                            origin: origin, 
                            direction: lower_left_corner + u * horizontal + v * vertical - origin
                        };

            let pix: Pixel = ray_to_pixel(r);
            write_pixel(pix);
        }
    }
}

fn main() {
    render_image_ppm(WIDTH, HEIGHT);
}

