extern crate dbsdk_rs;
use dbsdk_rs::{vdp, db, gamepad};

mod draw;
mod snake;
mod rng;
mod geometry;
mod util;

static mut GAME: Option<snake::Game> = None;
static mut CONTROLLER: Option<gamepad::Gamepad> = None;

fn tick() {
    let game = unsafe { GAME.as_mut().unwrap() };
    let controller = unsafe { CONTROLLER.as_mut().unwrap() };

    // this uses the controller only, can i use keyboard?
    if controller.is_connected() {
        let state = controller.read_state();

        if state.button_mask.contains(gamepad::GamepadButton::Up) {
            game.set_direction(snake::Direction::Up);
        } else if state.button_mask.contains(gamepad::GamepadButton::Down) {
            game.set_direction(snake::Direction::Down);
        } else if state.button_mask.contains(gamepad::GamepadButton::Left) {
            game.set_direction(snake::Direction::Left);
        } else if state.button_mask.contains(gamepad::GamepadButton::Right) {
            game.set_direction(snake::Direction::Right);
        }
    }

    match game.tick() {
        snake::TickResult::Win(_msg) => {
            
        }
        snake::TickResult::Lose(_msg) => {
            game.reset();
        }
        snake::TickResult::Continue => {
            game.draw();
        }
    }
}

#[no_mangle]
pub fn main(_: i32, _: i32) -> i32 {
    db::register_panic();
    vdp::depth_write(true);
    vdp::depth_func(vdp::Compare::LessOrEqual);
    
    unsafe {
        GAME = Some(snake::Game::new(10, 10, 3, 4, 8));
        CONTROLLER = Some(gamepad::Gamepad::new(gamepad::GamepadSlot::SlotA));
    }

    vdp::set_vsync_handler(Some(tick));

    return 0;
}
