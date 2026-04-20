use crate::{Ray, HitRecord, Color, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color
}

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz: f64::min(fuzz, 1.0) }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scattered_direction = rec.normal + Vec3::random_unit_vector();
        if scattered_direction.near_zero() {
            scattered_direction = rec.normal;
        }
        let attenuation = self.albedo;
        let scattered = Ray::new(rec.p, scattered_direction);
        Some((scattered, attenuation))
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = r_in.dir().reflect(rec.normal);
        reflected = reflected.unit_vector() + self.fuzz * Vec3::random_unit_vector();
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}