use std::fmt;
use std::ops;
// use std::cmp;
use super::num::ToPrimitive;

pub const ZERO_VECTOR: Vector2 = Vector2 { x: 0.0, y: 0.0 };
pub const UP_VECTOR: Vector2 = Vector2 { x: 0.0, y: 1.0 };
pub const RIGHT_VECTOR: Vector2 = Vector2 { x: 1.0, y: 0.0 };

#[derive(Default, PartialEq, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 {
            x: x,
            y: y
        }
    }

    pub fn unit_vector(&self) -> Self {
        use common::invsqrt;

        let sqr_sum = self.x*self.x + self.y*self.y;
        let scale = invsqrt(sqr_sum);
        Vector2 { 
            x: self.x * scale,
            y: self.y * scale
        }
    }
}

impl ops::Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Vector2) -> Self {
        Vector2 { 
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Vector2) -> Self {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T: ToPrimitive> ops::Mul<T> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let scale: f32 = rhs.to_f32().unwrap();
        Vector2 {
            x: self.x * scale,
            y: self.y * scale
        }
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {0}, y: {1}", self.x, self.y)
    }
}

pub fn dot_product(lhs: Vector2, rhs: Vector2) -> f32 {
    lhs.x * rhs.x + lhs.y * rhs.y
}

pub fn cross_product(lhs: Vector2, rhs: Vector2) -> f32 {
    lhs.x * rhs.x + lhs.y * rhs.y
}

pub fn distance(lhs: Vector2, rhs: Vector2) -> f32 {
    (rhs.x - lhs.x).sqrt() + (rhs.y - lhs.y).sqrt()
}
