mod camera;
mod hit;
mod ray;
mod sphere;
mod vec;
mod material;

use std::path::Path;

use bmp::{Image, Pixel};

use camera::Camera;
use hit::{Hit, World};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec::{Color, Point3, Vec3};
use material::{Lambertian, Metal};

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::rc::Rc;

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

fn ray_color(ray: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered_ray)) = rec.mat.scatter(ray, &rec) {
            return attenuation * ray_color(&scattered_ray, world, depth - 1)
        } else {
            return Color::new(0.0, 0.0, 0.0)
        }
    }

    let unit_direction = ray.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 512;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 5;

    // Multithreading
    const NUM_CORES: usize = 16;
    const WIDTH_PER_CORE: u32 = IMAGE_WIDTH / NUM_CORES as u32;

    let img = Arc::new(Mutex::new(Image::new(IMAGE_WIDTH, IMAGE_HEIGHT)));

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..NUM_CORES as u32 {
        let img = Arc::clone(&img);
        let handle = thread::spawn(move || {
            let camera = Camera::new(ASPECT_RATIO);

            // World
            let mut world = World::new();
            let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
            let mat_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
            let mat_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
            let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

            let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
            let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
            let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
            let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

            world.push(Box::new(sphere_ground));
            world.push(Box::new(sphere_center));
            world.push(Box::new(sphere_left));
            world.push(Box::new(sphere_right));

            let mut rng = rand::thread_rng();
            for x in (WIDTH_PER_CORE * i)..(WIDTH_PER_CORE * (i + 1)) {
                for y in 0..IMAGE_HEIGHT {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let random_u: f64 = rng.gen();
                        let random_v: f64 = rng.gen();
                        let (u, v) = (
                            (x as f64 + random_u) / (IMAGE_WIDTH - 1) as f64,
                            (y as f64 + random_v) / (IMAGE_HEIGHT - 1) as f64,
                        );
                        let ray = camera.get_ray(u, 1.0 - v);
                        pixel_color += ray_color(&ray, &world, MAX_DEPTH);
                    }
                    let mut image = img.lock().unwrap();
                    image.set_pixel(x, y, Pixel::from(pixel_color / SAMPLES_PER_PIXEL as f64));
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    /*for (x, y) in img.coordinates() {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES_PER_PIXEL {
            let random_u: f64 = rng.gen();
            let random_v: f64 = rng.gen();
            let (u, v) = (
                (x as f64 + random_u) / (IMAGE_WIDTH - 1) as f64,
                (y as f64 + random_v) / (IMAGE_HEIGHT - 1) as f64,
            );
            let ray = camera.get_ray(u, 1.0 - v);
            pixel_color += ray_color(&ray, &world, MAX_DEPTH);
        }

        img.set_pixel(x, y, Pixel::from(pixel_color / SAMPLES_PER_PIXEL as f64));
    }*/

    let path = Path::new("c:\\_work\\rusttracing\\img.bmp");
    img.lock().unwrap().save(path);
    println!("Success!");
}
