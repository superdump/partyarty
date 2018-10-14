use cgmath::{Point3, vec3, Vector3};

use ray::Ray;

pub struct Camera {
    origin: Point3<f32>,
    lower_left_corner: Point3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: Point3::new(0.0, 0.0, 0.0),
            lower_left_corner: Point3::new(-2.0, -1.0, -1.0),
            horizontal: vec3(4.0, 0.0, 0.0),
            vertical: vec3(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            (self.lower_left_corner + u * self.horizontal + v * self.vertical) - self.origin
        )
    }
}
