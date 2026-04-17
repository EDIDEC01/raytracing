use super::vector::Vec3;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub struct Point(pub Vec3);

impl Point {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }

    #[inline] pub fn x(&self) -> f32 { self.0.x }
    #[inline] pub fn y(&self) -> f32 { self.0.y }
    #[inline] pub fn z(&self) -> f32 { self.0.z }
}

// Delegate operations to Vec3
impl Add<Vec3> for Point {
    type Output = Point;
    fn add(self, rhs: Vec3) -> Self::Output { Point(self.0 + rhs) }
}

impl AddAssign<Vec3> for Point {
    fn add_assign(&mut self, rhs: Vec3) { self.0 += rhs; }
}

impl Sub<Vec3> for Point {
    type Output = Point;
    fn sub(self, rhs: Vec3) -> Self::Output { Point(self.0 - rhs) }
}

impl Sub<Point> for Point {
    type Output = Vec3;
    fn sub(self, rhs: Point) -> Self::Output { self.0 - rhs.0 }
}

impl SubAssign<Vec3> for Point {
    fn sub_assign(&mut self, rhs: Vec3) { self.0 -= rhs; }
}
