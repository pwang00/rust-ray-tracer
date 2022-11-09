use crate::camera::*;
use crate::color::*;
use crate::hittable_list::*;
use crate::ray::*;
use crate::render_params::*;
use crate::scene::*;
use crate::utilities::*;
use crate::vector::*;
use rayon::iter::ParallelIterator;

pub fn render_image_ppm(params: &RenderParams) {
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
    let scale = 1.0 / params.samples_per_pixel as f64;

    println!("P3\n{} {}\n255", width, height);
    for j in (-1..height as i32 - 1).rev() {
        for i in 0..width {
            let mut vec: VecR3 = VecR3::zero();
            for _ in 0..params.samples_per_pixel {
                let u: f64 = (i as f64 + Utils::random_double_01()) / (width - 1) as f64;
                let v: f64 = (j as f64 + Utils::random_double_01()) / (height - 1) as f64;
                let r: Ray = cam.get_ray(u, v);
                vec += Ray::vec_from_ray(r, &mut world, params.max_depth, params.tolerance);
            }
            write_pixel(vec.to_pixel(scale));
        }
    }
}

fn parallelized_render_ppm(params: &RenderParams) {
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
    let scale = 1.0 / params.samples_per_pixel as f64;

    let parallel_iter = (-1..height as i32 - 1).rev().zip(0..width);
}
