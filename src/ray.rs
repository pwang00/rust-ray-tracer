use crate::vector::VecR3;

pub type PointR3 = VecR3;

#[derive(Copy, Clone)]
pub struct Ray{
    pub origin : PointR3,
    pub direction : VecR3
}

impl Ray{
    pub fn point_at(&self, t: f64) -> PointR3{
        self.origin + t * self.direction
    }
}