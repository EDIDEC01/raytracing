use crate::{Point, Vec3};

#[derive(Clone)]
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
    #[inline] pub fn set_orig(&mut self, origin: Point) { self.origin = origin; }
    #[inline] pub fn set_dir(&mut self, direction: Vec3) { self.direction = direction; }

    pub fn at(&self, t: f64) -> Point {
        self.origin + (t * self.direction)
    }
}
