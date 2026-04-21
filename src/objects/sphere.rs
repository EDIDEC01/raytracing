use std::sync::Arc;

use crate::{HitRecord, Hittable, Point, Ray, Interval, Material, Vec3};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius: f64::max(0.0, radius),
            material: mat
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.orig();
        let a = r.dir().length_squared();
        let h = Vec3::dot(r.dir(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        
        let mut rec = HitRecord { 
            p, 
            normal: Vec3::default(), // temporaneo
            t: root, 
            front_face: false, // temporaneo
            material: Arc::clone(&self.material)
        };
        
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
