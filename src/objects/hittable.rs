use crate::types::point::Point;
use crate::types::vector::Vec3;
use crate::types::ray::Ray;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>; 
}