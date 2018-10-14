use cgmath::prelude::*;
use cgmath::{dot, Point3, Vector3};

use ray::Ray;

pub struct HitRecord {
    pub t: f32,
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
}

impl HitRecord {
    pub fn new(t: f32, p: Point3<f32>, normal: Vector3<f32>) -> HitRecord {
        HitRecord { t, p, normal }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3<f32>, radius: f32) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.magnitude2();
        let b = dot(oc, r.direction); // Removed factor of 2.0
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant_ish = b * b - a * c; // Removed factor of 4.0 on second term
        if discriminant_ish > 0.0 {
            let temp = (-b - discriminant_ish.sqrt()) / a;
            if t_min < temp && temp < t_max {
                let p = r.at_t(temp);
                return Some(HitRecord::new(temp, p, (p - self.center) / self.radius));
            }
            let temp = (-b + discriminant_ish.sqrt()) / a;
            if t_min < temp && temp < t_max {
                let p = r.at_t(temp);
                return Some(HitRecord::new(temp, p, (p - self.center) / self.radius));
            }
        }
        None
    }
}
