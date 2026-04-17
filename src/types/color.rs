use super::vector::Vec3;

pub type Color = Vec3;

impl Color {
    #[inline] pub fn r(&self) -> f32 { self.e[0] }
    #[inline] pub fn g(&self) -> f32 { self.e[1] }
    #[inline] pub fn b(&self) -> f32 { self.e[2] }

    pub fn write_color(&self) -> String {
        format!("{} {} {}\n", (self.r() * 255.999) as i32, (self.g() * 255.999) as i32, (self.b() * 255.999) as i32)
    }
}