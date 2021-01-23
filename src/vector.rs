use std::ops;

#[derive(Debug)]
pub struct Vector (f32, f32, f32);

impl ops::Add<&Vector> for &Vector{
    type Output = Vector;
    fn add(self, rhs: &Vector) -> Vector {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub<&Vector> for &Vector{
    type Output = Vector;
    fn sub(self, rhs: &Vector) -> Vector {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<&Vector> for &Vector {
    type Output = f32;
    fn mul (self, rhs: &Vector) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl ops::Mul<f32> for &Vector {
    type Output = Vector;
    fn mul (self, rhs: f32) -> Vector {
        Vector (
            self.0 * rhs,
            self.1 * rhs,
            self.2 * rhs,
        )
    }
}

impl ops::Div<f32> for &Vector{
    type Output = Vector;
    fn div(self, rhs: f32) -> Vector {
        Vector(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector(x,y,z)
    }

    pub fn origin() -> Self {
        Vector(0.0, 0.0, 0.0)
    }

    fn magnitude(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self / self.magnitude()
    }

}

pub fn cross(v: &Vector, w: &Vector) -> Vector {
    Vector (
        v.1 * w.2 - v.2 * w.1,
        v.2 * w.0 - v.0 * w.2,
        v.0 * w.1 - v.1 * w.0,
    )
}