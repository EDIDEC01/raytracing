use super::hittable::{HitRecord, Hittable};
use crate::types::point::Point;
use crate::types::ray::Ray;

pub struct Sphere {
    center: Point,
    radius: f64
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self {
            center,
            radius: f64::max(0.0, radius)
        }
    }

    #[inline] fn center(&self) -> Point { self.center }
    #[inline] fn radius(&self) -> f64 { self.radius }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - r.orig();
        let a = r.dir().length_squared();
        let h = r.dir().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let normal = (p - self.center) / self.radius;

        Some(
            HitRecord {
                p,
                normal,
                t
            }
        )
    }
}