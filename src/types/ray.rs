use super::point::Point;
use super::vector::Vec3;

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point, direction: Vec3) -> Self {
        Self {
            origin: origin,
            direction: direction,
        }
    }

    #[inline]
    pub fn orig(&self) -> Point {
        self.origin
    }
    #[inline]
    pub fn dir(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }
}
