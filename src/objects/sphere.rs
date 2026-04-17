use super::hittable::{HitRecord, Hittable};
use crate::types::point::Point;
use crate::types::ray::Ray;
use crate::types::vector::*;

pub struct Sphere {
    center: Point,
    radius: f32
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self {
            center,
            radius: f32::max(0.0, radius)
        }
    }

    #[inline] fn center(&self) -> Point { self.center }
    #[inline] fn radius(&self) -> f32 { self.radius }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let oc = self.center - r.orig();
        let a = r.dir().length_squared();
        let h = r.dir().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f32::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        Some(
            HitRecord {
                p: r.at(root),
                normal: (r.at(root) - self.center) / self.radius,
                t: root
            }
        )

    }
}