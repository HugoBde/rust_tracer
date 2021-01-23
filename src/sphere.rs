use crate::vector::Vector;

pub struct Sphere {
    center: Vector,
    radius: f32,
    color: (u8, u8, u8),
}

impl Sphere {

    pub fn new(center: Vector, radius: f32, color: (u8, u8, u8)) -> Self{
        Self{
            center: center,
            radius: radius,
            color: color
        }
    }

    pub fn intersect_distance(&self, ray: &Vector, ray_origin: &Vector) -> f32 {
        let c = &self.center + ray_origin;
        let p = &c * ray;
        let d_squared = &c * &c - p*p;
        let delta = (self.radius * self.radius - d_squared).sqrt();
        if p - delta > 0.0 {
            p - delta
        } else {
            if p + delta > 0.0 {
                p + delta
            } else {
                1000.0
            }
        }
    }

    pub fn color(&self) -> (u8,u8,u8) {
        self.color
    }
}