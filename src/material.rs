use glam::Vec3A;

use hitable::HitRecord;
use ray::Ray;
use utils::{random_float_01, random_in_unit_sphere};

fn reflect(v: &Vec3A, n: &Vec3A) -> Vec3A {
    *v - 2.0 * v.dot(*n) * *n
}

fn refract(v: &Vec3A, n: &Vec3A, ni_over_nt: f32) -> Option<Vec3A> {
    let uv = v.normalize();
    let dt = uv.dot(*n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        return Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt());
    }
    None
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r1 = r0 * r0;
    r1 + (1.0 - r1) * (1.0 - cosine).powf(5.0)
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3A,
}

pub fn lambertian(albedo: Vec3A) -> Material {
    Material::Lambertian(Lambertian { albedo })
}

#[derive(Clone, Copy)]
pub struct Metal{
    pub albedo: Vec3A,
    pub fuzz: f32,
}

pub fn metal(albedo: Vec3A, fuzz: f32) -> Material {
    Material::Metal(Metal { albedo, fuzz })
}

#[derive(Clone, Copy)]
pub struct Dielectric{
    pub ref_idx: f32,
}

pub fn dielectric(ref_idx: f32) -> Material {
    Material::Dielectric(Dielectric { ref_idx })
}

#[derive(Clone, Copy)]
pub enum Material {
    Dielectric(Dielectric),
    Lambertian(Lambertian),
    Metal(Metal),
}

pub fn scatter(r_in: &Ray, rec: &HitRecord) -> Option<(Vec3A, Ray)> {
    if let Some(material) = rec.material {
        match material {
            Material::Lambertian(m) => {
                let target = rec.p + rec.normal + random_in_unit_sphere();
                let scattered = Ray::new(rec.p, target - rec.p);
                return Some((m.albedo, scattered));
            },
            Material::Metal(m) => {
                let reflected = reflect(&r_in.direction.normalize(), &rec.normal);
                let scattered = Ray::new(rec.p, reflected + m.fuzz * random_in_unit_sphere());
                if scattered.direction.dot(rec.normal) > 0.0 {
                    return Some((m.albedo, scattered));
                } else {
                    return None;
                }
            },
            Material::Dielectric(m) => {
                let outward_normal;
                let ni_over_nt;
                let cosine;

                let rdn = r_in.direction.dot(rec.normal);
                if rdn > 0.0 {
                    outward_normal = -rec.normal;
                    ni_over_nt = m.ref_idx;
                    cosine = m.ref_idx * rdn / r_in.direction.length();
                } else {
                    outward_normal = rec.normal;
                    ni_over_nt = 1.0 / m.ref_idx;
                    cosine = -rdn / r_in.direction.length();
                }

                let attenuation = Vec3A::one();
                match refract(&r_in.direction, &outward_normal, ni_over_nt) {
                    Some(refracted) => {
                        if schlick(cosine, m.ref_idx) > random_float_01() {
                            return Some((
                                attenuation,
                                Ray::new(rec.p, reflect(&r_in.direction, &rec.normal))
                            ));
                        } else {
                            return Some((
                                attenuation,
                                Ray::new(rec.p, refracted)
                            ));
                        }
                    },
                    None => {
                        return Some((
                            attenuation,
                            Ray::new(rec.p, reflect(&r_in.direction, &rec.normal))
                        ));
                    }
                }
            },
        }
    }
    None
}
