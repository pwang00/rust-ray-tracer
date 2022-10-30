use std::{ops, fmt};

#[derive(Copy, Clone)]
pub struct Pixel{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl fmt::Display for Pixel{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{} {} {}", self.r, self.g, self.b)
    }
}

impl ops::Mul<f64> for Pixel{
    type Output = Pixel;

    fn mul(self, rhs: f64) -> Self::Output {
        Pixel{
            r: (self.r as f64 * rhs) as u8,
            g: (self.g as f64 * rhs) as u8,
            b: (self.b as f64 * rhs) as u8
        }
    }
}

impl ops::Mul<Pixel> for f64{
    type Output = Pixel;

    fn mul(self, rhs: Pixel) -> Self::Output {
        rhs * self
    }
}

pub fn write_pixel(p: Pixel){
    println!("{}", &p);
}