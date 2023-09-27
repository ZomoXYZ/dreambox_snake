extern crate dbsdk_rs;
use dbsdk_rs::{vdp::{self, Color32}, db};
use dbsdk_rs::math::Vector4;

fn box_tris(x: u8, y: u8, size: f32) -> Vec<vdp::Vertex> {
    let size_norm = size*2.0; // so size=1 is max

    let left = (x as f32) * size_norm - 1.0;
    let right = left + size_norm;
    let top = (y as f32) * size_norm - 1.0;
    let bottom = top + size_norm;

    // let pad = 0.05*size_norm;
    // let round = 0.1*size_norm;

    vec![
        vdp::Vertex::new(
            Vector4::new(left, top, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right, top, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right, bottom, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        
        vdp::Vertex::new(
            Vector4::new(left, top, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(left, bottom, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
        vdp::Vertex::new(
            Vector4::new(right, bottom, 0.0, 1.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::zero(),
            Vector4::zero()),
    ]
}

fn tick() {
    vdp::clear_color(Color32::new(0, 0, 0, 255));
    vdp::clear_depth(1.0);

    // draw a single triangle
    let triangles = box_tris(0, 0, 1.0);


    vdp::draw_geometry(vdp::Topology::TriangleList, &triangles);
}

#[no_mangle]
pub fn main(_: i32, _: i32) -> i32 {
    db::register_panic();
    vdp::depth_write(true);
    vdp::depth_func(vdp::Compare::LessOrEqual);
    vdp::set_vsync_handler(Some(tick));
    return 0;
}
