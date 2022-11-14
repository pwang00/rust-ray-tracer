use crate::camera::*;
use crate::hittable_list::*;
use crate::ray::*;
use crate::render_params::*;
use crate::scene::*;
use crate::utilities::*;
use crate::vector::*;
use image::ImageBuffer;
use image::Rgb;
use rayon::prelude::*;

pub fn unparallelized_render_image(params: &RenderParams) -> Option<ImageBuffer<Rgb<u8>, Vec<u8>>> {
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

    let width = params.width;
    let height = (params.width as f64 / params.aspect_ratio as f64) as u32;
    let scale = 1.0 / params.samples_per_pixel as f64;
    let pixel_map: Vec<u8> = (0..height)
        .into_iter()
        .map(|j| -> Vec<u8> {
            let mut row: Vec<u8> = Vec::with_capacity(width as usize);
            for i in 0..width {
                let mut vec: VecR3 = VecR3::zero();
                for _ in 0..params.samples_per_pixel {
                    let u: f64 = (i as f64 + Utils::random_double_01()) / (width - 1) as f64;
                    let v: f64 = (j as f64 + Utils::random_double_01()) / (height - 1) as f64;
                    let r: Ray = cam.get_ray(u, v);
                    vec += Ray::trace(r, &world, params.max_depth, params.tolerance);
                }
                row.extend_from_slice(&vec.to_pixel(scale))
            }
            row
        })
        .rev()
        .flatten()
        .collect();

    ImageBuffer::from_vec(width, height, pixel_map)
}

pub fn parallelized_render_image(params: &RenderParams) -> Option<ImageBuffer<Rgb<u8>, Vec<u8>>> {
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

    let width = params.width;
    let height = (params.width as f64 / params.aspect_ratio as f64) as u32;
    let scale = 1.0 / params.samples_per_pixel as f64;
    let pixel_map: Vec<u8> = (0..height)
        .into_par_iter()
        .map(|j| -> Vec<u8> {
            let mut row: Vec<u8> = Vec::with_capacity(width as usize);
            for i in 0..width {
                let mut vec: VecR3 = VecR3::zero();
                for _ in 0..params.samples_per_pixel {
                    let u: f64 = (i as f64 + Utils::random_double_01()) / (width - 1) as f64;
                    let v: f64 = (j as f64 + Utils::random_double_01()) / (height - 1) as f64;
                    let r: Ray = cam.get_ray(u, v);
                    vec += Ray::trace(r, &world, params.max_depth, params.tolerance);
                }
                row.extend_from_slice(&vec.to_pixel(scale))
            }
            row
        })
        .rev()
        .flatten()
        .collect();

    ImageBuffer::from_vec(width, height, pixel_map)
}
