use crate::color::*;
use std::io::Write;
use std::{fs::OpenOptions, vec::Vec};

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixel_map: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn new(width: u32, height: u32, pixel_map: Vec<Vec<Pixel>>) -> Self {
        Image {
            width,
            height,
            pixel_map,
        }
    }
}

pub fn write_ppm_to_file<'a>(ppm: &'a Image, filename: &'a str) -> Result<bool, &'a str> {
    let open_res = OpenOptions::new()
        .create(true)
        .write(true)
        .open(filename);

    let open_succeeded = match open_res {
        Ok(_) => true,
        Err(_) => false,
    };

    let mut file = open_res.unwrap();
    if !open_succeeded {
        return Err("Error opening file on disk");
    };

    // Write header
    writeln!(&mut file, "P3\n{} {}\n255", ppm.width, ppm.height).unwrap();

    for pixel in ppm.pixel_map.iter().flatten() {
        if let Err(_) = writeln!(&mut file, "{}", pixel) {
            return Err("Error occurred while writing line to file");
        }
    }

    Ok(open_succeeded)
}
