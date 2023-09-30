extern crate dbsdk_rs;

use dbsdk_rs::field_offset::offset_of;
use dbsdk_rs::vdp;
use dbsdk_rs::math::{Vector4, Matrix4x4, Vector3, Quaternion};

use geometry;

use crate::util::vec3_from;

pub fn transform_draw_tris(tris: &mut Vec<vdp::Vertex>, _frame: u32, _tick: u32) {
    Matrix4x4::load_identity_simd();

    let rotation = Matrix4x4::rotation(Quaternion::new(-0.34, 0.0, 0.05, 1.0));
    Matrix4x4::mul_simd(&rotation);

    let scale = Matrix4x4::scale(vec3_from(40.0));
    Matrix4x4::mul_simd(&scale);

    let position = Matrix4x4::translation(Vector3::new(20.0, 20.0, -60.0));
    Matrix4x4::mul_simd(&position);

    let projection = Matrix4x4::projection_perspective(640.0 / 480.0, 1.0, 0.1, 200.0);
    Matrix4x4::mul_simd(&projection);

    Matrix4x4::transform_vertex_simd(tris, offset_of!(vdp::Vertex => position));

    vdp::draw_geometry(vdp::Topology::TriangleList, &tris);
}

pub fn empty_box(other: &mut Vec<vdp::Vertex>, x: u8, y: u8, z: f32, size: f32) {
    let left = (x as f32) * size - 1.0;
    let right = left + size;
    let top = (y as f32) * size - 1.0;
    let bottom = top + size;

    let padding = 0.05 * size;
    
    let mut tris = geometry::square_tris([
        Vector4::new(left+padding, top+padding, z, 1.0),
        Vector4::new(right-padding, top+padding, z, 1.0),
        Vector4::new(right-padding, bottom-padding, z, 1.0),
        Vector4::new(left+padding, bottom-padding, z, 1.0)
    ], Vector4::new(0.3, 0.3, 0.3, 0.1));

    other.append(&mut tris);
}

pub fn food_box(other: &mut Vec<vdp::Vertex>, x: u8, y: u8, z: f32, size: f32) {
    let x1 = (x as f32) * size - 1.0;
    let x2 = x1 + size;
    let y1 = (y as f32) * size - 1.0;
    let y2 = y1 + size;
    let z1 = z;
    let z2 = z + size;

    let scale = vec3_from(1.0 / 3.0);
    let color = Vector4::new(1.0, 0.0, 0.0, 1.0);

    let c = geometry::Cube::new(x1, x2, y1, y2, z1, z2, scale, color, geometry::CubeWeight::Center);
    other.append(&mut c.tris());
}

pub fn head_box(other: &mut Vec<vdp::Vertex>, x: u8, y: u8, z: f32, size: f32, scale: f32) {
    let x1 = (x as f32) * size - 1.0;
    let x2 = x1 + size;
    let y1 = (y as f32) * size - 1.0;
    let y2 = y1 + size;
    let z1 = z;
    let z2 = z + size;

    let scale = vec3_from(0.95 * scale);
    let color = Vector4::new(0.4, 1.0, 0.4, 1.0);

    let c = geometry::Cube::new(x1, x2, y1, y2, z1, z2, scale, color, geometry::CubeWeight::Z1);
    other.append(&mut c.tris());
}

pub fn head_prediction_box(other: &mut Vec<vdp::Vertex>, x: i8, y: i8, z: f32, size: f32) {
    let x1 = (x as f32) * size - 1.0;
    let x2 = x1 + size;
    let y1 = (y as f32) * size - 1.0;
    let y2 = y1 + size;
    let z1 = z;
    let z2 = z + size;

    let scale = vec3_from(0.5);
    let color = Vector4::new(0.5, 1.0, 0.5, 1.0);

    let c = geometry::Cube::new(x1, x2, y1, y2, z1, z2, scale, color, geometry::CubeWeight::Z1);
    other.append(&mut c.tris());
}

pub fn body_box(other: &mut Vec<vdp::Vertex>, x: u8, y: u8, z: f32, size: f32, scale: f32) {
    let x1 = (x as f32) * size - 1.0;
    let x2 = x1 + size;
    let y1 = (y as f32) * size - 1.0;
    let y2 = y1 + size;
    let z1 = z;
    let z2 = z + size;

    let scale = vec3_from(0.85 * scale);
    let color = Vector4::new(0.0, 1.0, 0.0, 1.0);

    let c = geometry::Cube::new(x1, x2, y1, y2, z1, z2, scale, color, geometry::CubeWeight::Z1);
    other.append(&mut c.tris());
}

pub fn body_prediction_box(other: &mut Vec<vdp::Vertex>, x: i8, y: i8, z: f32, size: f32) {
    let x1 = (x as f32) * size - 1.0;
    let x2 = x1 + size;
    let y1 = (y as f32) * size - 1.0;
    let y2 = y1 + size;
    let z1 = z;
    let z2 = z + size;

    let scale = vec3_from(0.5);
    let color = Vector4::new(0.0, 1.0, 0.0, 1.0);

    let c = geometry::Cube::new(x1, x2, y1, y2, z1, z2, scale, color, geometry::CubeWeight::Z1);
    other.append(&mut c.tris());
}
