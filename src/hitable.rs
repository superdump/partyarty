use cgmath::prelude::*;
use cgmath::{dot, Point3, vec3, Vector3};
use specs::prelude::*;

use aabb::*;
use bvh::*;
use components::Position;
use material::Material;
use ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub t: f32,
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
    pub material: Option<Material>,
    pub entity: Option<Entity>,
}

impl HitRecord {
    pub fn new(
        t: f32,
        p: Point3<f32>,
        normal: Vector3<f32>,
        material: Option<Material>,
        entity: Option<Entity>,
    ) -> HitRecord {
        HitRecord { t, p, normal, material, entity }
    }
}

pub struct Sphere {
    pub radius: f32,
}

pub fn sphere(radius: f32) -> Hitable {
    Hitable::Sphere(Sphere { radius })
}

pub fn hit_sphere(
    position: &Position,
    sphere: &Sphere,
    r: &Ray,
    t_min: f32,
    t_max: f32,
) -> Option<HitRecord> {
    let position = position.0;
    let oc = r.origin - position;
    let a = r.direction.magnitude2();
    let b = dot(oc, r.direction); // Removed factor of 2.0
    let c = oc.magnitude2() - sphere.radius * sphere.radius;
    let discriminant_ish = b * b - a * c; // Removed factor of 4.0 on second term
    if discriminant_ish > 0.0 {
        let temp = (-b - discriminant_ish.sqrt()) / a;
        if t_min < temp && temp < t_max {
            let p = r.at_t(temp);
            return Some(HitRecord::new(temp, p, (p - position) / sphere.radius, None, None));
        }
        let temp = (-b + discriminant_ish.sqrt()) / a;
        if t_min < temp && temp < t_max {
            let p = r.at_t(temp);
            return Some(HitRecord::new(temp, p, (p - position) / sphere.radius, None, None));
        }
    }
    None
}

pub enum Hitable {
    BVHNode(BVHNode),
    Sphere(Sphere),
}

pub fn hit(
    tree: &BVHTree,
    positions: &ReadStorage<Position>,
    hitables: &ReadStorage<Hitable>,
    position: Option<&Position>,
    hitable: &Hitable,
    r: &Ray,
    t_min: f32,
    t_max: f32,
) -> Option<HitRecord> {
    match hitable {
        Hitable::BVHNode(h) => hit_bvh_node(tree, positions, hitables, h, r, t_min, t_max),
        Hitable::Sphere(h) => hit_sphere(position.unwrap(), h, r, t_min, t_max),
        _ => None,
    }
}

pub fn bounding_box(
    position: &Position,
    hitable: &Hitable,
    t_min: f32,
    t_max: f32
) -> Option<AABB> {
    match hitable {
        Hitable::BVHNode(h) => Some(h.aabb.clone()),
        Hitable::Sphere(h) => {
            let diagonal = vec3(h.radius, h.radius, h.radius);
            Some(AABB {
                min: position.0 - diagonal,
                max: position.0 + diagonal,
            })
        },
    }
}
