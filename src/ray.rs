use crate::vector::VecR3;

pub type PointR3 = VecR3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: PointR3,
    pub direction: VecR3,
}

impl Ray {
    pub fn zero() -> Self {
        Ray { origin: PointR3::zero(), direction: VecR3::zero() }
    }

    pub fn point_at(&self, t: f64) -> PointR3 {
        self.origin + t * self.direction
    }
}
