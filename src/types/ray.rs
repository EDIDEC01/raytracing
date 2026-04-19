use crate::point::Point;
use crate::vector::Vec3;

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    #[inline] pub fn orig(&self) -> Point { self.origin }
    #[inline] pub fn dir(&self) -> Vec3 { self.direction }

    pub fn at(&self, t: f64) -> Point {
        self.origin + (t * self.direction)
    }
}
