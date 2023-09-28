extern crate dbsdk_rs;
use dbsdk_rs::field_offset::offset_of;
use dbsdk_rs::vdp;
use dbsdk_rs::math::{Vector4, Matrix4x4, Vector3, Quaternion};

pub fn transform_draw_tris(tris: &mut Vec<vdp::Vertex>) {
    Matrix4x4::load_identity_simd();

    let s = 50.0;
    let scale = Matrix4x4::scale(Vector3::new(s,s,s));
    Matrix4x4::mul_simd(&scale);

    let rotation = Matrix4x4::rotation(Quaternion::new(-0.18, 0.0, 0.05, 1.0));
    Matrix4x4::mul_simd(&rotation);

    let position = Matrix4x4::translation(Vector3::new(0.0, 1.0, -40.0));
    Matrix4x4::mul_simd(&position);

    let proj = Matrix4x4::projection_perspective(640.0 / 480.0, 40.0, 0.1, 100.0);
    Matrix4x4::mul_simd(&proj);

    Matrix4x4::transform_vertex_simd(tris, offset_of!(vdp::Vertex => position));

    vdp::draw_geometry(vdp::Topology::TriangleList, &tris);
}

pub fn empty_box(other: &mut Vec<vdp::Vertex>, x: u8, y: u8, size: f32, z: f32) {
    let size_norm = size*2.0; // so size=1 is max

    let left = (x as f32) * size_norm - 1.0;
    let right = left + size_norm;
    let top = (y as f32) * size_norm - 1.0;
    let bottom = top + size_norm;

    let pad = 0.05*size_norm;
    let zbottom = z;

    let mut tris = vec![
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.3, 0.3, 0.3, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, zbottom, 1.0),
            Vector4::new(0.3, 0.3, 0.3, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.3, 0.3, 0.3, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.3, 0.3, 0.3, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.3, 0.3, 0.3, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.3, 0.3, 0.3, 1.0),
            Vector4::zero(),
            Vector4::zero()),
    ];
    other.append(&mut tris);
}

pub fn food_box(other: &mut Vec<vdp::Vertex>, x: u8, y: u8, size: f32, z: f32) {
    let size_norm = size*2.0; // so size=1 is max

    let left = (x as f32) * size_norm - 1.0;
    let right = left + size_norm;
    let top = (y as f32) * size_norm - 1.0;
    let bottom = top + size_norm;

    let pad = size_norm/3.0;
    let zbottom = z;

    let mut tris = vec![
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, zbottom, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, zbottom, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, zbottom, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, zbottom, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
    ];
    other.append(&mut tris);
}

pub fn head_box(other: &mut Vec<vdp::Vertex>, x: u8, y: u8, size: f32, z: f32) {
    let size_norm = size*2.0; // size=[0,1], so size_norm=[0,2] for [-1,1]

    let left = (x as f32) * size_norm - 1.0;
    let right = left + size_norm;
    let top = (y as f32) * size_norm - 1.0;
    let bottom = top + size_norm;

    let ztop = z + size_norm;
    let zbottom = z;

    let pad = 0.05*size_norm;

    let mut tris = vec![
        // bottom
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        // top
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        // left
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        // right
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        // front
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        // back
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.5, 1.0, 0.5, 1.0),
            Vector4::zero(),
            Vector4::zero()),
    ];
    other.append(&mut tris);
}

pub fn body_box(other: &mut Vec<vdp::Vertex>, x: u8, y: u8, size: f32, z: f32) {
    let size_norm = size*2.0; // size=[0,1], so size_norm=[0,2] for [-1,1]

    let left = (x as f32) * size_norm - 1.0;
    let right = left + size_norm;
    let top = (y as f32) * size_norm - 1.0;
    let bottom = top + size_norm;

    let pad = 0.05 * size_norm;
    let ztop = z + size_norm;
    let zbottom = z;

    let mut tris = vec![
        // bottom
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        // top
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        // left
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        // right
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        // front
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        // back
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, zbottom, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, ztop+pad*2.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
    ];
    other.append(&mut tris);
}
