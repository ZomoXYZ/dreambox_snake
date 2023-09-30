use dbsdk_rs::math::Vector3;

pub fn vec3_from(val: f32) -> Vector3 {
    Vector3::new(val, val, val)
}
