use cgmath::{Point3, vec3, Vector3};

use utils::point3;

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(o: Point3<f32>, d: Vector3<f32>) -> Ray {
        Ray {
            origin: o,
            direction: d,
        }
    }

    pub fn at_t(&self, t: f32) -> Point3<f32> {
        self.origin + self.direction * t
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray::new(
            point3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, 0.0),
        )
    }
}
