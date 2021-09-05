use std::fs::File;
use std::io::Write;
use std::path::Path;
mod vec;
use vec::{Vec3, Color, Point3};

fn printToFile(file: &mut File, string: &str)
{
    match file.write(format!("{}\n", string).as_bytes()) {
        Err(why) => panic!("Couldn't write to file: {}", why),
        Ok(_) => ()
    }
}

fn main() {
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

    let path = Path::new("c:\\_work\\rusttracing\\img.ppm");

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", path.display(), why),
        Ok(file) => file
    };

    printToFile(&mut file, "P3");
    printToFile(&mut file, format!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT).as_str());
    printToFile(&mut file, "255");

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25
            );
            
            printToFile(&mut file, pixel_color.format_color().as_str());
        }
    }
}
