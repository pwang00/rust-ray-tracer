use crate::camera::*;
use crate::color::*;
use crate::hittable_list::*;
use crate::image_formats::Image;
use crate::ray::*;
use crate::render_params::*;
use crate::scene::*;
use crate::utilities::*;
use crate::vector::*;
use rayon::prelude::*;

pub fn render_image_ppm(params: &RenderParams) -> Image {
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
    let mut pixel_map: Vec<Vec<Pixel>> =
        vec![vec![Pixel::default(); width as usize]; height as usize];

    for j in 0..height {
        for i in 0..width {
            let mut vec: VecR3 = VecR3::zero();
            for _ in 0..params.samples_per_pixel {
                let u: f64 = (i as f64 + Utils::random_double_01()) / (width - 1) as f64;
                let v: f64 = (j as f64 + Utils::random_double_01()) / (height - 1) as f64;
                let r: Ray = cam.get_ray(u, v);
                vec += Ray::trace(r, &mut world, params.max_depth, params.tolerance);
            }
            pixel_map[(height - 1 - j) as usize][i as usize] = vec.to_pixel(scale);
        }
    }

    return Image::new(width, height, pixel_map);
}

pub fn parallelized_render_ppm(params: &RenderParams) -> Image {
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
    let pixel_map: Vec<Vec<Pixel>> = (0..height)
        .into_par_iter()
        .map(|j| -> Vec<Pixel> {
            let mut row: Vec<Pixel> = Vec::with_capacity(width as usize);
            for i in 0..width {
                let mut vec: VecR3 = VecR3::zero();
                for _ in 0..params.samples_per_pixel {
                    let u: f64 = (i as f64 + Utils::random_double_01()) / (width - 1) as f64;
                    let v: f64 = (j as f64 + Utils::random_double_01()) / (height - 1) as f64;
                    let r: Ray = cam.get_ray(u, v);
                    vec += Ray::trace(r, &world, params.max_depth, params.tolerance);
                }
                row.push(vec.to_pixel(scale))
            }
            row
        })
        .rev()
        .collect();
    return Image::new(width, height, pixel_map);
}

