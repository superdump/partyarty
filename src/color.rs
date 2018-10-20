use cgmath::{Point3, Vector3};
use std::convert::From;
use std::fmt;
use std::ops::{AddAssign, Div, Mul, MulAssign};

#[derive(Clone, Copy, Debug, Default)]
pub struct Colorf32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Colorf32 {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Colorf32 {
        Colorf32 { r, g, b, a }
    }

    pub fn as_argb8888(&self) -> (u8, u8, u8, u8) {
        let c: Coloru8 = (*self).into();
        (c.a, c.r, c.g, c.b)
    }
}

impl fmt::Display for Colorf32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3}, {:.3})", self.r, self.g, self.b, self.a)
    }
}

impl AddAssign for Colorf32 {
    fn add_assign(&mut self, rhs: Colorf32) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl MulAssign<f32> for Colorf32 {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
    }
}

impl Mul<f32> for Colorf32 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Colorf32 {
        Colorf32::new(
            self.r * rhs,
            self.g * rhs,
            self.b * rhs,
            self.a * rhs,
        )
    }
}

impl Mul<Colorf32> for f32 {
    type Output = Colorf32;

    fn mul(self, rhs: Colorf32) -> Colorf32 {
        Colorf32::new(
            self * rhs.r,
            self * rhs.g,
            self * rhs.b,
            self * rhs.a,
        )
    }
}

impl Mul<Colorf32> for Vector3<f32> {
    type Output = Colorf32;

    fn mul(self, rhs: Colorf32) -> Colorf32 {
        Colorf32::new(
            self.x * rhs.r,
            self.y * rhs.g,
            self.z * rhs.b,
            rhs.a,
        )
    }
}

impl Div<f32> for Colorf32 {
    type Output = Self;

    fn div(self, rhs: f32) -> Colorf32 {
        self * (1.0f32 / rhs)
    }
}

pub struct Coloru8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Coloru8 {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Coloru8 {
        Coloru8 { r, g, b, a }
    }
}

impl From<Coloru8> for u32 {
    fn from(color: Coloru8) -> Self {
        ((color.a as u32) << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
    }
}

impl From<Colorf32> for Coloru8 {
    fn from(color: Colorf32) -> Self {
        Coloru8 {
            r: (255.99 * color.r.sqrt()) as u8,
            g: (255.99 * color.g.sqrt()) as u8,
            b: (255.99 * color.b.sqrt()) as u8,
            a: (255.99 * color.a) as u8,
        }
    }
}

impl From<Colorf32> for u32 {
    fn from(color: Colorf32) -> Self {
        Coloru8::from(color).into()
    }
}

impl From<Point3<f32>> for Colorf32 {
    fn from(color: Point3<f32>) -> Self {
        Colorf32::new(color.x, color.y, color.z, 1.0)
    }
}

impl From<Vector3<f32>> for Colorf32 {
    fn from(color: Vector3<f32>) -> Self {
        Colorf32::new(color.x, color.y, color.z, 1.0)
    }
}
