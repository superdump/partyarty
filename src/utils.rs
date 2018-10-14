use cgmath::Vector3;

pub fn lerp_vec3(a: Vector3<f32>, t: f32, b: Vector3<f32>) -> Vector3<f32> {
    (1.0 - t) * a + t * b
}
