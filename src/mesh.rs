use crate::vector::{Vector, cross};

pub struct Triangle{
    v0: Vector,
    v1: Vector,
    v2: Vector,
    normal: Vector,
    d: f32,
    color: (u8, u8, u8),
}

impl Triangle {
    pub fn new(v0: Vector, v1: Vector, v2: Vector, color: (u8, u8, u8)) -> Self{
        let v0v1 = &v1 - &v0;
        let v0v2 = &v2 - &v0;
        let normal = cross(&v0v1,&v0v2);
        let d = &normal * &v0;
        Self {
            v0: v0,
            v1: v1,
            v2: v2,
            normal: normal,
            d: d,
            color: color,
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        self.color
    }

    pub fn intersect_distance(&self, ray: &Vector, ray_origin: &Vector) -> f32 {
        if &self.normal * ray == 0.0 {
            1000.0
        } else {
            let t = ((&self.normal * ray_origin) + self.d) / (&self.normal * ray);

            if t < 0.0 {
                1000.0
            } else {
                let v = &(ray * t) + ray_origin;

                let edge0 = &self.v1 - &self.v0;
                let vp0 = &v - &self.v0;
                let c = cross(&edge0, &vp0);
                if &self.normal * &c < 0.0 {
                    return 1000.0;
                }
                let edge1 = &self.v2 - &self.v1;
                let vp1 = &v - &self.v1;
                let c = cross(&edge1, &vp1);
                if &self.normal * &c < 0.0 {
                    return 1000.0;
                }
                let edge2 = &self.v0 - &self.v2;
                let vp2 = &v - &self.v2;
                let c = cross(&edge2, &vp2);
                if &self.normal * &c < 0.0 {
                    return 1000.0;
                }
                t
            }
        }
    }
}

pub struct Mesh {
    triangles: Vec<Triangle>,
}