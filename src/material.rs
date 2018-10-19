use cgmath::Vector3;
use cgmath::prelude::*;

use hitable::HitRecord;
use ray::Ray;
use utils::random_in_unit_sphere;

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(*n) * n
}

#[derive(Clone, Copy)]
pub struct Lambertian(pub Vector3<f32>);

#[derive(Clone, Copy)]
pub struct Metal(pub Vector3<f32>);

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

pub fn scatter(r_in: &Ray, rec: &HitRecord, state: &mut u32) -> Option<(Vector3<f32>, Ray)> {
    if let Some(material) = rec.material {
        match material {
            Material::Lambertian(m) => {
                let target = rec.p + rec.normal + random_in_unit_sphere(state);
                let scattered = Ray::new(rec.p, target - rec.p);
                return Some((m.0, scattered));
            },
            Material::Metal(m) => {
                let reflected = reflect(&r_in.direction.normalize(), &rec.normal);
                let scattered = Ray::new(rec.p, reflected);
                if scattered.direction.dot(rec.normal) > 0.0 {
                    return Some((m.0, scattered));
                } else {
                    return None;
                }
            },
        }
    }
    None
}
