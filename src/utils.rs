use rand::{Rng, RngCore, SeedableRng, CryptoRng, Error};
use rand::rngs::SmallRng;
use glam::Vec3A;

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

pub fn lerp_vec3(a: Vec3A, t: f32, b: Vec3A) -> Vec3A {
    (1.0 - t) * a + t * b
}

pub fn random_in_unit_disk() -> Vec3A {
    let mut p;
    let one_one_zero = Vec3A::new(1.0f32, 1.0f32, 0.0f32);
    // the below is a do {} while () loop
    while {
        p = 2.0 * Vec3A::new(random_float_01(), random_float_01(), 0.0) - one_one_zero;
        p.length_squared() >= 1.0
    } {}
    p
}

pub fn random_in_unit_sphere() -> Vec3A {
    let mut p;
    let ones = Vec3A::new(1.0f32, 1.0f32, 1.0f32);
    // the below is a do {} while () loop
    while {
        p = 2.0 * Vec3A::new(random_float_01(), random_float_01(), random_float_01()) - ones;
        p.length_squared() >= 1.0
    } {}
    p
}
