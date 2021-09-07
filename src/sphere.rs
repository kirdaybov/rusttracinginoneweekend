use super::hit::{Hit, HitRecord};
use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64) -> Sphere {
        Sphere{
            center: cen,
            radius: r
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-b - sqrtd) / (2.0 * a);
        if root < t_min || root > t_max {
             root = (-b + sqrtd) / (2.0 * a);
             if root < t_min || root > t_max {
                 return None
             }
        }

        let outward_normal = (ray.at(root) - self.center).normalized();
        let mut rec = HitRecord {
            p: ray.at(root),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: root,
            front_face: false
        };
        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }
}
