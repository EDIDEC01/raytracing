use super::vector::Vec3;
use std::io::{Result, Write};
use std::ops::{Add, Mul};

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub struct Color(pub Vec3);

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }

    #[inline] pub fn r(&self) -> f64 { self.0.x }
    #[inline] pub fn g(&self) -> f64 { self.0.y }
    #[inline] pub fn b(&self) -> f64 { self.0.z }

    pub fn write_color<W: Write>(&self, writer: &mut W) -> Result<()> {
        writeln!(
            writer,
            "{} {} {}",
            (self.r() * 255.999) as i32,
            (self.g() * 255.999) as i32,
            (self.b() * 255.999) as i32
        )
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output { Color(self.0 + rhs.0) }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output { Color(self.0 * rhs) }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output { Color(self * rhs.0) }
}
