// only 2-dimensional for now

use std::ops::Add;
use std::ops::Mul;

pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn add(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn sub(&mut self, other: &Vector) {
        let mut other_cpy = other.copy();
        other_cpy.scale(-1.0);
        self.add(&other_cpy)
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn scale(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
    }

    pub fn copy(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
        }
    }

    // view as 3-dim vectors (x, y, 0) and return z
    pub fn cross(&self, other: &Vector) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Vector {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.dot(&other)
    }
}

/*
impl Mul for Vector {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        let res = &self.copy();
        res.scale(f);
        res
    }
}
*/
