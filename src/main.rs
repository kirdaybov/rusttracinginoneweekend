use std::fs::File;
use std::io::Write;
use std::path::Path;
mod vec;
mod ray;
use ray::Ray;
use vec::{Vec3, Color, Point3};
use bmp::{Image, Pixel};

fn printToFile(file: &mut File, string: &str)
{
    match file.write(format!("{}\n", string).as_bytes()) {
        Err(why) => panic!("Couldn't write to file: {}", why),
        Ok(_) => ()
    }
}

impl From<Color> for Pixel {
    fn from(pixel: Color) -> Pixel {
        Pixel { 
            r: (pixel.index_int(0) as u8),
            g: (pixel.index_int(1) as u8),
            b: (pixel.index_int(2) as u8) 
        }
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let left_corner = Point3::new(- viewport_width/2.0, -viewport_height/2.0, -1.0);

    let path = Path::new("c:\\_work\\rusttracing\\img.bmp");

    let mut img = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let origin = Vec3::new( 0.0, 0.0, 0.0);

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

        img.set_pixel(x, y, Pixel::from(ray_color(&ray)));
    }

    img.save(path);
    println!("Success!");
}
