use cgmath::{Point3, vec3, Vector3};
use cgmath::prelude::*;
use rand::{FromEntropy, Rng, RngCore, CryptoRng, Error};
use rand::rngs::SmallRng;

use std::cell::UnsafeCell;

thread_local! {
    static SMALLRNG: UnsafeCell<SmallRng> = UnsafeCell::new(SmallRng::from_entropy());
}

pub struct ThreadSmallRng {
    rng: *mut SmallRng,
}

pub fn thread_small_rng() -> ThreadSmallRng {
    ThreadSmallRng { rng: SMALLRNG.with(|t| t.get()) }
}

impl RngCore for ThreadSmallRng {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        unsafe { (*self.rng).next_u32() }
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        unsafe { (*self.rng).next_u64() }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        unsafe { (*self.rng).fill_bytes(dest) }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        unsafe { (*self.rng).try_fill_bytes(dest) }
    }
}

impl CryptoRng for ThreadSmallRng {}

pub fn random_float_01() -> f32 {
    thread_small_rng().gen()
}

pub fn lerp_vec3(a: Vector3<f32>, t: f32, b: Vector3<f32>) -> Vector3<f32> {
    (1.0 - t) * a + t * b
}

pub fn point3(x: f32, y: f32, z: f32) -> Point3<f32> {
    Point3::new(x, y, z)
}

pub fn random_in_unit_disk() -> Vector3<f32> {
    let mut p;
    let one_one_zero = vec3(1.0f32, 1.0f32, 0.0f32);
    // the below is a do {} while () loop
    while {
        p = 2.0 * vec3(random_float_01(), random_float_01(), 0.0) - one_one_zero;
        p.magnitude2() >= 1.0
    } {}
    p
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    let mut p;
    let ones = vec3(1.0f32, 1.0f32, 1.0f32);
    // the below is a do {} while () loop
    while {
        p = 2.0 * vec3(random_float_01(), random_float_01(), random_float_01()) - ones;
        p.magnitude2() >= 1.0
    } {}
    p
}
