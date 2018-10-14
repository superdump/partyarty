use cgmath::{Point3, Vector3};
use std::convert::From;

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
            r: (255.99 * color.r) as u8,
            g: (255.99 * color.g) as u8,
            b: (255.99 * color.b) as u8,
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
