use cgmath::Point3;

use ray::Ray;
use utils::point3;

use std::fmt;

#[derive(Clone)]
pub struct AABB {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl AABB {
    pub fn new(
        x0: f32, y0: f32, z0: f32,
        x1: f32, y1: f32, z1: f32,
    ) -> AABB {
        AABB {
            min: point3(x0, y0, z0),
            max: point3(x1, y1, z1),
        }
    }
}

impl fmt::Display for AABB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "AABB: ({:.3}, {:.3}, {:.3}) -> ({:.3}, {:.3}, {:.3})",
            self.min.x, self.min.y, self.min.z,
            self.max.x, self.max.y, self.max.z
        )
    }
}

impl Default for AABB {
    fn default() -> AABB {
        AABB::new(
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        )
    }
}

pub fn hit_aabb(
    aabb: &AABB,
    r: &Ray,
    t_min: f32,
    t_max: f32,
) -> bool {
    for a in 0..3 {
        let inv_d = 1.0 / r.direction[a];
        let mut t0 = (aabb.min[a] - r.origin[a]) * inv_d;
        let mut t1 = (aabb.max[a] - r.origin[a]) * inv_d;
        if inv_d < 0.0 {
            let temp = t0;
            t0 = t1;
            t1 = temp;
        }
        t0 = f32::max(t_min, t0);
        t1 = f32::min(t_max, t1);
        if t0 >= t1 {
            return false;
        }
    }
    true
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    AABB::new(
        f32::min(box0.min.x, box1.min.x),
        f32::min(box0.min.y, box1.min.y),
        f32::min(box0.min.z, box1.min.z),
        f32::max(box0.max.x, box1.max.x),
        f32::max(box0.max.y, box1.max.y),
        f32::max(box0.max.z, box1.max.z),
    )
}
