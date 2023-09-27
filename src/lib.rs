extern crate dbsdk_rs;
use dbsdk_rs::{vdp::{self, Color32, Vertex}, db};
use std::cmp::min;

mod draw;
mod snake;
mod rand;

static mut GAME: Option<snake::Game> = None;

fn draw(game: &mut snake::Game) {
    vdp::clear_color(Color32::new(0, 0, 0, 255));
    vdp::clear_depth(1.0);

    let mut tris = Vec::<Vertex>::new();

    let size = 1.0 / (min(game.width, game.height) as f32);

    for x in 0..game.width {
        for y in 0..game.height {
            match game.at(x, y) {
                snake::Location::Head(_) => {
                    let mut t = draw::box_tris(x, y, size);
                    tris.append(&mut t);
                }
                snake::Location::Body(_) => {
                    let mut t = draw::box_tris(x, y, size);
                    tris.append(&mut t);
                }
                snake::Location::Food => {
                    let mut t = draw::box_tris(x, y, size);
                    tris.append(&mut t);
                }
                snake::Location::Empty => {
                    
                }
            }
        }
    }

    vdp::draw_geometry(vdp::Topology::TriangleList, &tris);
}

fn tick() {
    unsafe {
        let game = GAME.as_mut().unwrap();
        match game.tick() {
            snake::TickResult::Win(_msg) => {
                
            }
            snake::TickResult::Lose(_msg) => {
                
            }
            snake::TickResult::Continue => {
                draw(game);
            }
        }
    }
}

#[no_mangle]
pub fn main(_: i32, _: i32) -> i32 {
    db::register_panic();
    vdp::depth_write(true);
    vdp::depth_func(vdp::Compare::LessOrEqual);
    
    unsafe {
        GAME = Some(snake::Game::new(10, 10, 3, 4, 6));
    }

    vdp::set_vsync_handler(Some(tick));

    return 0;
}
