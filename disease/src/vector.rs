// only 2-dimensional for now

use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }

    // view as 3-dim vectors (x, y, 0) and return z
    pub fn cross(&self, other: &Vector) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn square(&self) -> f64 {
        self.dot(&self)
    }

    pub fn mag(&self) -> f64 {
        self.square().sqrt()
    }

    pub fn normalized(&self) -> Vector {
        *self * (1.0 / self.mag())
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<Vector> for Vector {
    type Output = f64;

    fn mul(self, other: Self) -> Self::Output {
        self.dot(&other)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, f: f64) -> Self::Output {
        Vector {
            x: self.x * f,
            y: self.y * f
        }
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, f: f64) {
        *self = *self * f;
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, f: f64) -> Self::Output {
        self * (1.0/f)
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, f: f64) {
        *self = *self / f;
    }
}
