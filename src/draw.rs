extern crate dbsdk_rs;

use dbsdk_rs::field_offset::offset_of;
use dbsdk_rs::vdp;
use dbsdk_rs::math::{Vector4, Matrix4x4, Vector3, Quaternion};

use geometry::cube::Cube;

use crate::geometry::square::Square;
use crate::geometry::weight::{CENTER, self};
use crate::util::{vec3_from, vec3};

pub fn transform_draw_tris(tris: &mut Vec<vdp::Vertex>, _frame: u32, _tick: u32) {
    Matrix4x4::load_identity_simd();

    let rotation = Matrix4x4::rotation(Quaternion::new(-0.34, 0.0, 0.05, 1.0));
    Matrix4x4::mul_simd(&rotation);

    let scale = Matrix4x4::scale(vec3_from(40.0));
    Matrix4x4::mul_simd(&scale);

    let position = Matrix4x4::translation(Vector3::new(-20.0, -20.0, -40.0));
    Matrix4x4::mul_simd(&position);

    let projection = Matrix4x4::projection_perspective(640.0 / 480.0, 1.0, 0.1, 200.0);
    Matrix4x4::mul_simd(&projection);

    Matrix4x4::transform_vertex_simd(tris, offset_of!(vdp::Vertex => position));

    vdp::draw_geometry(vdp::Topology::TriangleList, &tris);
}

pub fn floor_box(other: &mut Vec<vdp::Vertex>, x: f32, y: f32, z: f32, size: f32, color: Vector4) {
    let from = vec3(
        x * size,
        y * size,
        z * size
    );
    let to = vec3(
        (x + 1.0) * size,
        (y + 1.0) * size,
        z * size
    );
    let scale = vec3_from(0.9);

    let square = Square::new(from, to, scale, color, CENTER);
    other.append(&mut square.tris());
}

pub fn food_box(other: &mut Vec<vdp::Vertex>, x: f32, y: f32, z: f32, size: f32) {
    let from = vec3(
        x * size,
        y * size,
        z * size
    );
    let to = vec3(
        (x + 1.0) * size,
        (y + 1.0) * size,
        (z + 1.0) * size
    );
    let scale = vec3_from(1.0 / 3.0);
    let color = Vector4::new(1.0, 0.0, 0.0, 1.0);

    let c = Cube::new(from, to, scale, color, weight::CENTER);
    other.append(&mut c.tris());
}

pub fn body_box(other: &mut Vec<vdp::Vertex>, head: bool, x: f32, y: f32, z: f32, size: f32, scale: f32) {
    let from = vec3(
        x * size,
        y * size,
        z * size
    );
    let to = vec3(
        (x + 1.0) * size,
        (y + 1.0) * size,
        (z + 1.0) * size
    );
    let scale = vec3_from(if head { 0.95 } else { 0.85 } * scale);
    let color = if head { Vector4::new(0.4, 1.0, 0.4, 1.0) } else { Vector4::new(0.0, 1.0, 0.0, 1.0) };

    let c = Cube::new(from, to, scale, color, weight::Z1);
    other.append(&mut c.tris());
}

pub fn body_prediction_box(other: &mut Vec<vdp::Vertex>, head: bool, x: f32, y: f32, z: f32, size: f32, weight: Vector3) {
    let from = vec3(
        x * size,
        y * size,
        z * size
    );
    let to = vec3(
        (x + 1.0) * size,
        (y + 1.0) * size,
        (z + 1.0) * size
    );
    let scale: Vector3 = vec3_from(0.5);
    let color = if head { Vector4::new(0.4, 1.0, 0.4, 1.0) } else { Vector4::new(0.0, 1.0, 0.0, 1.0) };

    let c = Cube::new(from, to, scale, color, weight);
    other.append(&mut c.tris());
}
