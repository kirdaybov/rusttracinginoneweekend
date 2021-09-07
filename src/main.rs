mod vec;
mod ray;
mod hit;
mod sphere;

use std::path::Path;

use bmp::{Image, Pixel};

use vec::{Vec3, Color, Point3};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;

impl From<Color> for Pixel {
    fn from(pixel: Color) -> Pixel {
        Pixel { 
            r: (pixel.index_int(0) as u8),
            g: (pixel.index_int(1) as u8),
            b: (pixel.index_int(2) as u8) 
        }
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt())/(2.0*a) //TODO: check negative
    }
}

fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0))/2.0;
    }

    let unit_direction = ray.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let origin = Vec3::new( 0.0, 0.0, 0.0);
    let left_corner = Point3::new(- viewport_width/2.0, -viewport_height/2.0, -1.0);

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut img = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    
    for (x, y) in img.coordinates() {
        let (u, v) = (
            x as f64 / (IMAGE_WIDTH - 1) as f64,
            y as f64 / (IMAGE_HEIGHT - 1) as f64
        );
        let ray = Ray::new(
            origin,
            left_corner + Vec3::new(
                u * viewport_width,
                (1.0 - v) * viewport_height,
                0.0
            )
        );

        img.set_pixel(x, y, Pixel::from(ray_color(&ray, &world)));
    }

    let path = Path::new("c:\\_work\\rusttracing\\img.bmp");
    img.save(path);
    println!("Success!");
}
