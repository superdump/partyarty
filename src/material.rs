use cgmath::Vector3;
use cgmath::prelude::*;

use hitable::HitRecord;
use ray::Ray;
use utils::random_in_unit_sphere;

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(*n) * n
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

pub fn lambertian(albedo: Vector3<f32>) -> Material {
    Material::Lambertian(Lambertian { albedo })
}

#[derive(Clone, Copy)]
pub struct Metal{
    pub albedo: Vector3<f32>,
    pub fuzz: f32,
}

pub fn metal(albedo: Vector3<f32>, fuzz: f32) -> Material {
    Material::Metal(Metal { albedo, fuzz })
}

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
                return Some((m.albedo, scattered));
            },
            Material::Metal(m) => {
                let reflected = reflect(&r_in.direction.normalize(), &rec.normal);
                let scattered = Ray::new(rec.p, reflected + m.fuzz * random_in_unit_sphere(state));
                if scattered.direction.dot(rec.normal) > 0.0 {
                    return Some((m.albedo, scattered));
                } else {
                    return None;
                }
            },
        }
    }
    None
}
