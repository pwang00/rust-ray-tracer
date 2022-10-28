pub struct Pixel{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub fn write_pixel(p: Pixel){
    print!("{} {} {}\n", p.r, p.g, p.b);
}