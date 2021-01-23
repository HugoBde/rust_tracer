use crate::sphere::Sphere;
use crate::mesh::Triangle;
use crate::vector::Vector;

use std::fs::{File};
use std::io::Write;

const WIDTH  : usize = 512;
const HEIGHT : usize = 512;
const FOV : f32 = std::f32::consts::FRAC_PI_3;



pub fn render() -> [(u8, u8, u8);WIDTH * HEIGHT] {
    let t = Triangle::new(
        Vector::new(-3.0, 1.0, -2.0),
        Vector::new(3.0, 1.0, -2.0),
        Vector::new(-3.0, 1.0, -10.0),
        (255,178,129),
    );
    let t2 = Triangle::new(
        Vector::new(3.0, 1.0, -2.0),
        Vector::new(3.0, 1.0, -10.0),
        Vector::new(-3.0, 1.0, -10.0),
        (178,255,129),
    );
    let s = Sphere::new(
        Vector::new(0.0, -1.0, -4.0),
        1.0,
        (178,129,255)
    );
    let mut output: [(u8, u8, u8); WIDTH * HEIGHT] = [(255,255,255);WIDTH * HEIGHT];
    for i in 0..HEIGHT {
        let y = (FOV/2.0).tan() * (((i as f32 + 0.5)/ HEIGHT as f32) - 0.5) * 2.0;
        for j in 0..WIDTH {
            let x = (FOV/2.0).tan() * (((j as f32 + 0.5)/ HEIGHT as f32) - 0.5) * 2.0;
            let v = Vector::new( x, y, -1.0).normalize();
            let mut pixel_color = (255, 255, 255);
            let mut closest_hit = 1000.0;
            let last_hit = t.intersect_distance(&v, &Vector::origin());
            if last_hit < closest_hit {
                pixel_color = t.color();
                closest_hit = last_hit;
            }
            let last_hit = t2.intersect_distance(&v, &Vector::origin());
            if last_hit < closest_hit {
                pixel_color = t2.color();
                closest_hit = last_hit;
            }
            let last_hit = s.intersect_distance(&v, &Vector::origin());
            if last_hit < closest_hit {
                pixel_color = s.color();
                closest_hit = last_hit;
            }
            output[i * WIDTH+ j] = pixel_color;
        }
    }
    output
}

pub fn save(arr: [(u8, u8,u8);WIDTH * HEIGHT]) -> Result<(),std::io::Error> {
    let mut output = File::create("output.ppm")?;
    output.write(format!("P3\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes())?;
    let mut buffer = String::new();
    for i in 0..WIDTH*HEIGHT {
        let (r,g,b) = arr[i];
        buffer.push_str(&format!("{} {} {} ",r,g,b));
    }
    output.write(buffer.as_bytes())?;
    Ok(())
}