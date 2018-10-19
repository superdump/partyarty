use cgmath::{Point3, vec3, Vector3};
use cgmath::prelude::*;

fn xor_shift_32(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 15;
    *state = x;
    x
}

pub fn random_float_01(state: &mut u32) -> f32 {
    (xor_shift_32(state) & 0xFFFFFF) as f32 / 16777216.0f32
}

pub fn lerp_vec3(a: Vector3<f32>, t: f32, b: Vector3<f32>) -> Vector3<f32> {
    (1.0 - t) * a + t * b
}

pub fn point3(x: f32, y: f32, z: f32) -> Point3<f32> {
    Point3::new(x, y, z)
}

pub fn random_in_unit_sphere(state: &mut u32) -> Vector3<f32> {
    let mut p;
    let ones = vec3(1.0f32, 1.0f32, 1.0f32);
    // the below is a do {} while () loop
    while {
        p = 2.0 * vec3(random_float_01(state), random_float_01(state), random_float_01(state)) - ones;
        p.magnitude2() >= 1.0
    } {}
    p
}
