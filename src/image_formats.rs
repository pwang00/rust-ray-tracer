
use std::{vec::Vec, fs::File, path::Path};
use crate::color::*;
pub struct P3PPM{
    pub height: u32,
    pub width: u32,
    pub pixel_map: Vec<Pixel>,
}

fn write_ppm_to_file<'a, 'b>(ppm: &'a P3PPM, filename: &'b str) -> Result<bool, &'a str>{
    let file: File;

    if !Path::new(filename).exists(){
        match File::create(filename){
            Ok(f) => file = f,
            Err(e) => return Err("Error while trying to create file: {e}"),
        }
    }

    Ok(true)
}