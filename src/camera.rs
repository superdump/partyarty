use cgmath::{Point3, vec3, Vector3};
use cgmath::prelude::*;
use std::default::Default;
use std::f32;

use ray::Ray;
use utils::{point3, random_in_unit_disk};

#[derive(Debug)]
pub struct Camera {
    origin: Point3<f32>,
    lower_left_corner: Point3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,
    lens_radius: f32,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera::new(
            point3(-2.0, 2.0, 1.0),
            point3(0.0, 0.0, -1.0),
            vec3(0.0, 1.0, 0.0),
            90.0,
            2.0,
            0.1,
            10.0,
        )
    }
}

impl Camera {
    pub fn new(
        look_from: Point3<f32>,
        look_at: Point3<f32>,
        vup: Vector3<f32>,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (0.5 * theta).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            origin: look_from,
            lower_left_corner:
                look_from
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u, v, w,
            lens_radius: 0.5 * aperture,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = rd.x * self.u + rd.y * self.v;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner
                + s * self.horizontal
                + t * self.vertical
                - self.origin
                - offset,
        )
    }
}
