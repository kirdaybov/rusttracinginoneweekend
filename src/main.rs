use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::fmt;
use std::fmt::Display;

fn printToFile(file: &mut File, string: &str)
{
    match file.write(format!("{}\n", string).as_bytes()) {
        Err(why) => panic!("Couldn't write to file: {}", why),
        Ok(_) => ()
    }
}

#[derive(Clone, Copy)]
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

    pub fn x(self) -> f64 {
        self[0]
    }

    pub fn y(self) -> f64 {
        self[1]
    }

    pub fn z(self) -> f64 {
        self[2]
    }

    pub fn dot(self, other : Vec3) -> f64 {
        self[0]*other[0] + self[1]*other[1] + self[2]*other[2]
    }

    pub fn cross(self, other : Vec3) -> Vec3 {
        Vec3 { 
            e : [
                self[1]*other[2] - self[2]*other[1],
                self[2]*other[0] - self[0]*other[2],
                self[0]*other[1] - self[1]*other[0]
            ]
        }
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn format_color(self) -> String {
        format!("{} {} {}", 
            (255.99 * self[0]) as u64,
            (255.99 * self[1]) as u64,
            (255.99 * self[2]) as u64,
        )
    }
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

impl Display for Vec3 {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
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
