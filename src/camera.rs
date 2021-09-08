use super::vec::{Point3, Vec3};
use super::ray::Ray;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(aspect_ratio : f64) -> Camera {
        const FOCAL_LENGTH: f64 = 1.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        let viewport_width = VIEWPORT_HEIGHT * aspect_ratio;
        
        let orig = Point3::new(0.0, 0.0, 0.0);
        let h = Vec3::new(viewport_width, 0.0, 0.0);
        let v = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let llc = orig - h/2.0 - v/2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
        
        Camera {
            origin: orig,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin,self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
