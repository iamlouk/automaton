// only 2-dimensional for now

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn add(&mut self, other: &Vector) -> &mut Vector {
        self.x += other.x;
        self.y += other.y;
        self
    }

    pub fn sub(&mut self, other: &Vector) -> &mut Vector {
        self.add(&other.copy().scale(-1.0))
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn scale(&mut self, other: f64) -> &mut Vector {
        self.x *= other;
        self.y *= other;
        self
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

    pub fn square(&self) -> f64 {
        self.dot(&self)
    }

    pub fn mag(&self) -> f64 {
        self.square().sqrt()
    }

    pub fn normalize(&mut self) -> &mut Vector {
        self.scale(1.0 / self.mag());
        self
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

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
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
impl Mul for &Vector {
    type Output = Vector;

    fn mul(self, f: f64) -> Vector {
        let res = self.copy();
        res.scale(f);
        res
    }
}
*/
