extern crate dbsdk_rs;
use dbsdk_rs::vdp::Vertex;
use dbsdk_rs::math::Vector4;

pub fn box_tris(x: u8, y: u8, size: f32) -> Vec<Vertex> {
    let size_norm = size*2.0; // so size=1 is max

    let left = (x as f32) * size_norm - 1.0;
    let right = left + size_norm;
    let top = (y as f32) * size_norm - 1.0;
    let bottom = top + size_norm;

    let pad = 0.05*size_norm;
    // let round = 0.1*size_norm;

    vec![
        Vertex::new(
            Vector4::new(left+pad, top+pad, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        Vertex::new(
            Vector4::new(right-pad, top+pad, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        Vertex::new(
            Vector4::new(right-pad, bottom-pad, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        Vertex::new(
            Vector4::new(left+pad, top+pad, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        Vertex::new(
            Vector4::new(left+pad, bottom-pad, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        Vertex::new(
            Vector4::new(right-pad, bottom-pad, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
    ]
}
