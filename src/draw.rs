extern crate dbsdk_rs;
use dbsdk_rs::vdp;
use dbsdk_rs::math::Vector4;

pub fn empty_box(other: &mut Vec<vdp::Vertex>, x: u8, y: u8, size: f32) {
    let size_norm = size*2.0; // so size=1 is max

    let left = (x as f32) * size_norm - 1.0;
    let right = left + size_norm;
    let top = (y as f32) * size_norm - 1.0;
    let bottom = top + size_norm;

    let pad = 0.05*size_norm;

    let mut tris = vec![
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, 0.0, 1.0),
            Vector4::new(0.3, 0.3, 0.3, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, 0.0, 1.0),
            Vector4::new(0.3, 0.3, 0.3, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, 0.0, 1.0),
            Vector4::new(0.3, 0.3, 0.3, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, 0.0, 1.0),
            Vector4::new(0.3, 0.3, 0.3, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, 0.0, 1.0),
            Vector4::new(0.3, 0.3, 0.3, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, 0.0, 1.0),
            Vector4::new(0.3, 0.3, 0.3, 1.0),
            Vector4::zero(),
            Vector4::zero()),
    ];
    other.append(&mut tris);
}

pub fn food_box(other: &mut Vec<vdp::Vertex>, x: u8, y: u8, size: f32) {
    let size_norm = size*2.0; // so size=1 is max

    let left = (x as f32) * size_norm - 1.0;
    let right = left + size_norm;
    let top = (y as f32) * size_norm - 1.0;
    let bottom = top + size_norm;

    let pad = size_norm/3.0;

    let mut tris = vec![
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
    ];
    other.append(&mut tris);
}

pub fn head_box(other: &mut Vec<vdp::Vertex>, x: u8, y: u8, size: f32) {
    let size_norm = size*2.0; // size=[0,1], so size_norm=[0,2] for [-1,1]

    let left = (x as f32) * size_norm - 1.0;
    let right = left + size_norm;
    let top = (y as f32) * size_norm - 1.0;
    let bottom = top + size_norm;

    let pad = 0.05*size_norm;

    let mut tris = vec![
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, 0.0, 1.0),
            Vector4::new(0.4, 1.0, 0.4, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, 0.0, 1.0),
            Vector4::new(0.4, 1.0, 0.4, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, 0.0, 1.0),
            Vector4::new(0.4, 1.0, 0.4, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, 0.0, 1.0),
            Vector4::new(0.4, 1.0, 0.4, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, 0.0, 1.0),
            Vector4::new(0.4, 1.0, 0.4, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, 0.0, 1.0),
            Vector4::new(0.4, 1.0, 0.4, 1.0),
            Vector4::zero(),
            Vector4::zero()),
    ];
    other.append(&mut tris);
}

pub fn body_box(other: &mut Vec<vdp::Vertex>, x: u8, y: u8, size: f32) {
    let size_norm = size*2.0; // size=[0,1], so size_norm=[0,2] for [-1,1]

    let left = (x as f32) * size_norm - 1.0;
    let right = left + size_norm;
    let top = (y as f32) * size_norm - 1.0;
    let bottom = top + size_norm;

    let pad = 0.05*size_norm;

    let mut tris = vec![
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, 0.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, top+pad, 0.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, 0.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left+pad, top+pad, 0.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left+pad, bottom-pad, 0.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right-pad, bottom-pad, 0.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
    ];
    other.append(&mut tris);
}
