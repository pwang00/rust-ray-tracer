use crate::vector::VecR3;

type Point = VecR3;

#[derive(Copy, Clone)]
pub struct Ray{
    pub origin : Point,
    pub direction : VecR3
}

pub trait PointAt{
    fn point_at(&self, t: f64) -> Point;
}

// WIP
impl PointAt for Ray{
    fn point_at(&self, t: f64) -> Point{
        self.origin + t * self.direction
    }
}