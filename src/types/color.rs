use crate::{Vec3, Interval};
use std::io::{Result, Write};
use std::ops::{Add, Mul, AddAssign};

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub struct Color(pub Vec3);

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }

    #[inline]
    pub fn r(&self) -> f64 {
        self.0.x
    }
    #[inline]
    pub fn g(&self) -> f64 {
        self.0.y
    }
    #[inline]
    pub fn b(&self) -> f64 {
        self.0.z
    }

    pub fn write_color<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut r = Color::linear_to_gamma(self.r());
        let mut g = Color::linear_to_gamma(self.g());
        let mut b = Color::linear_to_gamma(self.b());

        let intensity = Interval::new(0.0, 0.999);

        r = 256.0 * intensity.clamp(r);
        g = 256.0 * intensity.clamp(g);
        b = 256.0 * intensity.clamp(b);

        writer.write_all(&[r as u8, g as u8, b as u8])
    }

    fn linear_to_gamma(linear_component: f64) -> f64{
        if linear_component > 0.0 {
            f64::sqrt(linear_component)
        } else {
            0.0
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Self) -> Self::Output {
        Color(self.0 * rhs.0)
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color(self.0 * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color(self * rhs.0)
    }
}
