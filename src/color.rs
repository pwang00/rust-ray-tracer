use std::io;

pub struct Pixel{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

fn write_color(p: Pixel){
    print!("{} {} {}\n", p.r, p.g, p.b);
}