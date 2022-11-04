use crate::color::*;
use crate::utilities::*;
use std::{fmt, ops};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct VecR3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Vector operations
impl fmt::Display for VecR3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Neg for VecR3 {
    type Output = VecR3;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl ops::Add<VecR3> for VecR3 {
    type Output = VecR3;

    fn add(self, rhs: VecR3) -> Self::Output {
        VecR3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<VecR3> for VecR3 {
    type Output = VecR3;

    fn sub(self, rhs: VecR3) -> Self::Output {
        VecR3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f64> for VecR3 {
    type Output = VecR3;

    fn mul(self, rhs: f64) -> Self::Output {
        VecR3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<VecR3> for f64 {
    type Output = VecR3;

    fn mul(self, rhs: VecR3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<VecR3> for VecR3 {
    type Output = VecR3;

    fn mul(self, rhs: VecR3) -> Self::Output {
        self.hadamard_product(rhs)
    }
}


impl ops::Div<f64> for VecR3 {
    type Output = VecR3;

    fn div(self, rhs: f64) -> Self::Output {
        VecR3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::AddAssign<VecR3> for VecR3 {
    fn add_assign(&mut self, rhs: VecR3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl VecR3 {
    pub const fn zero() -> Self {
        VecR3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn hadamard_product(self, other: VecR3) -> VecR3 {
        VecR3{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }

    pub fn is_near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot_product(self, other: VecR3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(self, other: VecR3) -> VecR3 {
        VecR3 {
            x: (self.y * other.z - self.z * other.y),
            y: -(self.x * other.z - self.z * other.y),
            z: (self.x * other.y - self.y * other.x),
        }
    }

    pub fn normalize(&self) -> VecR3 {
        let norm: f64 = self.norm();
        VecR3 {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }

    pub fn to_pixel(&self, scale: f64) -> Pixel {
        Pixel {
            r: (256.0 * clamp((self.x * scale).sqrt(), 0.0, 0.999)) as u8,
            g: (256.0 * clamp((self.y * scale).sqrt(), 0.0, 0.999)) as u8,
            b: (256.0 * clamp((self.z * scale).sqrt(), 0.0, 0.999)) as u8,
        }
    }

    pub fn random_vec_01() -> VecR3 {
        VecR3 {
            x: random_double_01(),
            y: random_double_01(),
            z: random_double_01(),
        }
    }

    pub fn random_vec(min: f64, max: f64) -> VecR3 {
        VecR3 {
            x: random_double(min, max),
            y: random_double(min, max),
            z: random_double(min, max),
        }
    }
}

pub fn random_unit_vector() -> VecR3 {
    return random_in_unit_sphere().normalize();
}

pub fn random_in_unit_sphere() -> VecR3 {
    loop {
        let v: VecR3 = VecR3::random_vec(-1.0, 1.0);
        if v.norm().powi(2) >= 1.0 {
            continue;
        }

        return v;
    }
}

pub fn random_in_hemisphere(nv: VecR3) -> VecR3 {
    let in_unit_sphere: VecR3 = random_in_unit_sphere();
    let sgn: f64 = if in_unit_sphere.dot_product(nv) > 0.0 {
        1.0
    } else {
        -1.0
    };
    return sgn * in_unit_sphere;
}

pub fn reflect(v: VecR3, n: VecR3) -> VecR3 {
    v - 2.0 * v.dot_product(n) * n
}