extern crate dbsdk_rs;
extern crate getrandom;
use dbsdk_rs::{vdp::{self, Color32, Vertex}, db};

mod draw;
// mod snake;

fn tick() {
    vdp::clear_color(Color32::new(0, 0, 0, 255));
    vdp::clear_depth(1.0);

    // draw a single triangle
    let mut tris = Vec::<Vertex>::new();

    for x in 0..10 {
        for y in 0..10 {
            let mut t = draw::box_tris(x, y, 0.1);
            tris.append(&mut t);
        }
    }

    vdp::draw_geometry(vdp::Topology::TriangleList, &tris);
}

#[no_mangle]
pub fn main(_: i32, _: i32) -> i32 {
    db::register_panic();
    vdp::depth_write(true);
    vdp::depth_func(vdp::Compare::LessOrEqual);
    
    // let snake = snake::Game::new(10, 10, 1000.0/15.0);

    vdp::set_vsync_handler(Some(tick));

    // tick();

    return 0;
}
