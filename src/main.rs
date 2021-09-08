mod camera;
mod hit;
mod ray;
mod sphere;
mod vec;

use std::path::Path;

use bmp::{Image, Pixel};

use camera::Camera;
use hit::{Hit, World};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec::{Color, Point3};

impl From<Color> for Pixel {
    fn from(pixel: Color) -> Pixel {
        Pixel {
            r: (pixel.index_int(0) as u8),
            g: (pixel.index_int(1) as u8),
            b: (pixel.index_int(2) as u8),
        }
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a) //TODO: check negative
    }
}

fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) / 2.0;
    }

    let unit_direction = ray.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u64 = 100;

    let camera = Camera::new(ASPECT_RATIO);

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut img = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut rng = rand::thread_rng();

    for (x, y) in img.coordinates() {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES_PER_PIXEL {
            let random_u: f64 = rng.gen();
            let random_v: f64 = rng.gen();
            let (u, v) = (
                (x as f64 + random_u) / (IMAGE_WIDTH - 1) as f64,
                (y as f64 + random_v) / (IMAGE_HEIGHT - 1) as f64,
            );
            let ray = camera.get_ray(u, 1.0 - v);
            pixel_color += ray_color(&ray, &world);
        }

        img.set_pixel(x, y, Pixel::from(pixel_color / SAMPLES_PER_PIXEL as f64));
    }

    let path = Path::new("c:\\_work\\rusttracing\\img.bmp");
    img.save(path);
    println!("Success!");
}
