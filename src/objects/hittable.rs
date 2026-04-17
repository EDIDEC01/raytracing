use crate::types::point::Point;
use crate::types::vector::Vec3;
use crate::types::ray::Ray;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>; 
}