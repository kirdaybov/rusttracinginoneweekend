use super::ray::Ray;
use super::vec::{Color, Vec3};
use super::hit::HitRecord;
use rand::Rng;

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

pub struct Dielectric {
    ir: f64
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0/self.ir
        } else {
            self.ir
        };
        let unit_vector = ray.direction().normalized();
        let cos_theta = ((-1.0) * unit_vector).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = sin_theta * refraction_ratio > 1.0;
        let will_reflect = rng.gen::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            unit_vector.reflect(rec.normal)
        } else {
            unit_vector.refract(rec.normal, refraction_ratio)
        };
        Some((Color::new(1.0, 1.0, 1.0), Ray::new(rec.p, direction)))
    }
}