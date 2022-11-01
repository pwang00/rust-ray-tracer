use std::{ops, fmt};

pub const SAMPLES_PER_PIXEL: u32 = 100;
pub const SCALE: f64 = 1.0 / SAMPLES_PER_PIXEL as f64;
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

impl ops::Add<Pixel> for Pixel{
    type Output = Self;

    fn add(self, rhs: Pixel) -> Self::Output {
        Pixel{r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b}
    }
}

impl ops::Mul<Pixel> for f64{
    type Output = Pixel;

    fn mul(self, rhs: Pixel) -> Self::Output {
        rhs * self
    }
}

#[inline(always)]
fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x < min {return min}
    if x > max {return max}
    return x
}

pub fn write_pixel(p: Pixel){
    let (r, g, b) = (
        SCALE * p.r as f64, 
        SCALE * p.g as f64, 
        SCALE * p.b as f64
    );

    // Ensure that pixel values are bounded between 0 and 256
    let cx = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let cy = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let cz = (256.0 * clamp(b, 0.0, 0.999)) as u8;

    println!("{cx} {cy} {cz}");
}