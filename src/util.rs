use dbsdk_rs::math::{Vector3, Vector4};

// built in min/max uses Ord which f32 doesn't satisfy
pub fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
pub fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

pub fn vec3_from(val: f32) -> Vector3 {
    Vector3::new(val, val, val)
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3::new(x, y, z)
}

pub fn vec3_to4(v: Vector3) -> Vector4 {
    Vector4::new(v.x, v.y, v.z, 1.0)
}
