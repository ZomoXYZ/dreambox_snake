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
        let deadzone = 0.2;

        let gamepad = util::read_gamepad(state);
        let left_stick = util::read_deadzone(deadzone, state.left_stick_x, state.left_stick_y);
        let right_stick = util::read_deadzone(deadzone, state.right_stick_x, state.right_stick_y);
        
        if let Some(dir) = gamepad {
            game.set_direction(dir);
        } else if let Some(dir) = left_stick {
            game.set_direction(dir);
        } else if let Some(dir) = right_stick {
            game.set_direction(dir);
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
        GAME = Some(snake::Game::new(12, 12, 3, 4, 8));
        CONTROLLER = Some(gamepad::Gamepad::new(gamepad::GamepadSlot::SlotA));
    }

    vdp::set_vsync_handler(Some(tick));

    return 0;
}
