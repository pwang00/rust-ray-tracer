use crate::camera::*;
use crate::color::*;
use crate::hittable_list::*;
use crate::ray::*;
use crate::render_params::*;
use crate::scene::*;
use crate::utilities::*;
use crate::vector::*;
use itertools::Itertools;
use rayon::prelude::*;

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

fn parallelized_render_ppm(params: &RenderParams) -> Vec<Vec<Pixel>> {
    let world: HittableList = random_scene();

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

    let vec: Vec<(u32, u32)> = (0..=height).cartesian_product(0..width).collect();

    let vec2: Vec<(u32, u32, Pixel)> = vec
        .into_par_iter()
        .map(|(i, j)| -> (u32, u32, Pixel) {
            let mut vec: VecR3 = VecR3::zero();
            for _ in 0..params.samples_per_pixel {
                let u: f64 = (i as f64 + Utils::random_double_01()) / (width - 1) as f64;
                let v: f64 = (j as f64 + Utils::random_double_01()) / (height - 1) as f64;
                let r: Ray = cam.get_ray(u, v);
                vec += Ray::vec_from_ray(r, &world, params.max_depth, params.tolerance);
            }
            (i, j, vec.to_pixel(scale))
        })
        .collect();

    let mut pixel_map: Vec<Vec<Pixel>> =
        vec![vec![Pixel::default(); width as usize]; height as usize];

    for (i, j, p) in vec2.iter() {
        pixel_map[*i as usize][*j as usize] = *p;
    }

    return pixel_map;
}
