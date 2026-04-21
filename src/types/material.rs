use crate::{Color, HitRecord, Ray, Vec3, random_f64};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
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

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: f64::min(fuzz, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = Vec3::reflect(r_in.dir(), rec.normal);
        reflected = Vec3::unit_vector(reflected) + self.fuzz * Vec3::random_unit_vector();
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_dir = Vec3::unit_vector(r_in.dir());
        let cos_theta = f64::min(Vec3::dot(-unit_dir, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let should_reflect =
            ri * sin_theta > 1.0 || Self::reflectance(cos_theta, ri) > random_f64();

        let direction = if should_reflect {
            Vec3::reflect(unit_dir, rec.normal)
        } else {
            Vec3::refract(unit_dir, rec.normal, ri)
        };

        let scattered = Ray::new(rec.p, direction);
        let attenuation = Color::new(1.0, 1.0, 1.0);

        Some((scattered, attenuation))
    }
}
