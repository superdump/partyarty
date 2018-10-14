use cgmath::Vector3;

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
