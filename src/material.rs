use super::ray::Ray;
use super::vec::{Color, Vec3};
use super::hit::HitRecord;

pub trait Scatter {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian {
            albedo: a
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalized();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered_ray = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered_ray))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Metal {
        Metal {
            albedo: a,
            fuzz: f
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.direction().reflect(rec.normal).normalized();
        let scattered_ray = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        if scattered_ray.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered_ray))
        } else {
            None
        }
    }
}