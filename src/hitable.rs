use glam::Vec3A;

use components::Position;
use material::Material;
use ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3A,
    pub normal: Vec3A,
    pub material: Option<Material>,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3A, normal: Vec3A, material: Option<Material>) -> HitRecord {
        HitRecord { t, p, normal, material }
    }
}

pub struct Sphere {
    pub radius: f32,
}

pub fn sphere(radius: f32) -> Hitable {
    Hitable::Sphere(Sphere { radius })
}

pub enum Hitable {
    Sphere(Sphere),
}

pub fn hit(position: &Position, hitable: &Hitable, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    match hitable {
        Hitable::Sphere(h) => {
            let oc = r.origin - position.0;
            let a = r.direction.length_squared();
            let b = oc.dot(r.direction); // Removed factor of 2.0
            let c = oc.length_squared() - h.radius * h.radius;
            let discriminant_ish = b * b - a * c; // Removed factor of 4.0 on second term
            if discriminant_ish > 0.0 {
                let temp = (-b - discriminant_ish.sqrt()) / a;
                if t_min < temp && temp < t_max {
                    let p = r.at_t(temp);
                    return Some(HitRecord::new(temp, p, (p - position.0) / h.radius, None));
                }
                let temp = (-b + discriminant_ish.sqrt()) / a;
                if t_min < temp && temp < t_max {
                    let p = r.at_t(temp);
                    return Some(HitRecord::new(temp, p, (p - position.0) / h.radius, None));
                }
            }
            None
        }
    }
}
