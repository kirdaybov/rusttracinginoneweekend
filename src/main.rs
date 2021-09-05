use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

fn printToFile(file: &mut File, string: &str)
{
    match file.write(format!("{}\n", string).as_bytes()) {
        Err(why) => panic!("Couldn't write to file: {}", why),
        Ok(_) => ()
    }
}

struct Vec3 {
    e: [f64; 3]
}

type Point3 = Vec3;
type Color = Vec3;

impl Vec3 {
    fn new(e0 : f64, e1 : f64, e2 : f64) -> Vec3 {
        Vec3 { 
            e: [e0, e1, e2] 
        }
    }

    //fn length(&self) -> f64 {
    //    (self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)).sqrt()
    //}
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { 
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
         }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { 
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
         }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { 
            e: [self[0]*other[0], self[1]*other[1], self[2]*other[2]]
         }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3 { 
            e: [self[0]*other, self[1]*other, self[2]*other]
         }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [self[0]*other, self[1]*other, self[2]*other]
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        Vec3 { 
            e: [self[0]/other, self[1]/other, self[2]/other]
         }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [self[0]/other, self[1]/other, self[2]/other]
        }
    }
}

fn main() {
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

    let path = Path::new("D:\\_work\\rust\\rtinoneweekend\\img.ppm");

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

    for j in 0..IMAGE_WIDTH {
        for i in 0..IMAGE_HEIGHT {
            let r = i as f64 / (IMAGE_HEIGHT - 1) as f64;
            let g = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let b = 0.25;
            let ir = (255.99 * r) as u64;
            let ig = (255.99 * g) as u64;
            let ib = (255.99 * b) as u64;
            printToFile(&mut file, format!("{} {} {}", ir, ig, ib).as_str());
        }
    }
}
