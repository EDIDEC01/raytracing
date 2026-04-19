pub mod camera;
pub mod objects;
pub mod types;

pub use camera::*;
pub use objects::*;
pub use types::*;

pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f64 = std::f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    use rand::RngExt;
    rand::rng().random::<f64>()
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
