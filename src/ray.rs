use glam::Vec3A;

pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
}

impl Ray {
    pub fn new(o: Vec3A, d: Vec3A) -> Ray {
        Ray {
            origin: o,
            direction: d,
        }
    }

    pub fn at_t(&self, t: f32) -> Vec3A {
        self.origin + self.direction * t
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray::new(
            Vec3A::zero(),
            Vec3A::zero(),
        )
    }
}
