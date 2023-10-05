use std::cmp::max;
use dbsdk_rs::{vdp, math::Vector4};

use rng;
use draw;

use crate::{util::{vec3, vec3_rand, vec3_from}, geometry::floaty::{StateFloaty, FloatyCamera}};

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    pub width: u8,
    pub height: u8,
    table: Vec<i16>,

    state_floaty: Vec<StateFloaty>,
    floaty_camera: FloatyCamera,
    
    pub size: u16,
    last_direction: Direction,
    direction: Direction,
    pub head: [u8; 2],

    interval_frames: u32,
    interval_frame: u32,
    frame: u32,
    tick: u32,

    rng: rng::Rng,
    last_tick: TickResult<String, String>,
}

#[derive(Clone)]
pub enum TickResult<W, L> {
    Win(W),
    Lose(L),
    Continue
}

#[derive(Clone)]
pub enum Location {
    Head(u16),
    Body(u16),
    Food,
    Empty,
}

impl Game {
    pub fn new(width: u8, height: u8, left: u8, top: u8, interval_frames: u32) -> Game {
        let mut rng = rng::Rng::new();

        let mut table = Vec::with_capacity((width * height).into());
        for _ in 0..width*height {
            table.push(0);
        }

        let mut state_floaty = Vec::with_capacity(((width+2)*(height+2)).into());
        for _ in 0..(width+2)*(height+2) {
            state_floaty.push(StateFloaty::new(
                vec3_rand(&mut rng, 90, 240),
                vec3_rand(&mut rng, 0, 120),
                vec3_from(0.3)
            ));
        }

        let mut game = Game {
            width,
            height,
            table,

            state_floaty,
            floaty_camera: FloatyCamera::new(&mut rng,
                vec3(5.0, 5.0, 1.0),
                vec3(0.15, 0.05, 0.08),
                10 * 60, 25 * 60,
                0, 15 * 60
            ),
            
            size: 1,
            last_direction: Direction::Right,
            direction: Direction::Right,
            head: [left-1, height-top], // should this be rng?

            interval_frames,
            interval_frame: 0,
            frame: 0,
            tick: 0,

            rng,
            last_tick: TickResult::Continue,
        };
        let _ = game.new_food(); // rng will be consistent if i call it here
        return game
    }

    pub fn reset(&mut self) {
        self.size = 1;
        // self.last_direction = Direction::Right;
        // self.direction = Direction::Right;
        self.head = [self.width/2, self.height/2];
        self.interval_frame = 0;
        self.frame = 0;
        self.tick = 0;
        self.last_tick = TickResult::Continue;
        self.table = Vec::with_capacity((self.width * self.height).into());
        for _ in 0..self.width*self.height {
            self.table.push(0);
        }
        let _ = self.new_food();
    }

    pub fn set_direction(&mut self, direction: Direction) {
        // prevent turning back on ourselves
        if matches!(direction, Direction::Up) && matches!(self.last_direction, Direction::Down) {
            return;
        }
        if matches!(direction, Direction::Down) && matches!(self.last_direction, Direction::Up) {
            return;
        }
        if matches!(direction, Direction::Left) && matches!(self.last_direction, Direction::Right) {
            return;
        }
        if matches!(direction, Direction::Right) && matches!(self.last_direction, Direction::Left) {
            return;
        }
        self.direction = direction;
    }

    pub fn at(&self, x: u8, y: u8) -> Location {
        if x == self.head[0] && y == self.head[1] {
            return Location::Head(self.size)
        }
        let val = self.get(x, y);
        if val == -1 {
            return Location::Food
        }
        if val > 0 {
            return Location::Body(val as u16)
        }
        Location::Empty
    }

    fn get_state_floaty_index(&self, x: i8, y: i8) -> usize {
        let x = (x + 1) as u8;
        let y = (y + 1) as u8;
        if x >= (self.width + 2) || y >= (self.height + 2) {
            return 0
        }
        (y as usize) * ((self.width + 2) as usize) + (x as usize)
    }
    pub fn tick_state_floaty(&mut self, x: i8, y: i8) -> StateFloaty {
        let index = self.get_state_floaty_index(x, y);
        self.state_floaty[index].tick();
        self.state_floaty[index]
    }

    fn get_index(&self, x: u8, y: u8) -> usize {
        if x >= self.width || y >= self.height {
            return 0
        }
        (y as usize) * (self.width as usize) + (x as usize)
    }
    fn get(&self, x: u8, y: u8) -> i16 {
        self.table[self.get_index(x, y)]
    }
    fn set(&mut self, x: u8, y: u8, value: i16) {
        let index = self.get_index(x, y);
        self.table[index] = value;
    }

    fn new_food(&mut self) -> Result<(), &str> {
        let pos = self.rng.random(self.width);

        let index = self.get_index(pos[0], pos[1]);
        let mut off = 0;

        loop {
            if self.table[index+off] == 0 {
                break;
            }
            if off >= (self.width * self.height).into() {
                return Err("No space for food")
            }
            off+=1
        }

        self.table[index+off] = -1;

        Ok(())
    }

    pub fn tick(&mut self) -> TickResult<String, String> {
        self.tick += 1;

        if !matches!(self.last_tick, TickResult::Continue) {
            return self.last_tick.clone()
        }

        if self.interval_frame < self.interval_frames {
            self.interval_frame += 1;
            return TickResult::Continue
        }
        self.interval_frame = 0;
        self.frame += 1;

        let result = self.tick_internal();
        self.last_tick = result.clone();
        result

        // TickResult::Continue
    }

    fn tick_internal(&mut self) -> TickResult<String, String> {
        // set head
        self.set(self.head[0], self.head[1], self.size as i16);

        // move head
        match self.direction {
            Direction::Up => {
                if self.head[1] == self.height - 1 {
                    self.head[1] = 0;
                } else {
                    self.head[1] += 1;
                }
            },
            Direction::Down => {
                if self.head[1] == 0 {
                    self.head[1] = self.height - 1;
                } else {
                    self.head[1] -= 1;
                }
            },
            Direction::Left => {
                if self.head[0] == 0 {
                    self.head[0] = self.width - 1;
                } else {
                    self.head[0] -= 1;
                }
            },
            Direction::Right => {
                if self.head[0] == self.width - 1 {
                    self.head[0] = 0;
                } else {
                    self.head[0] += 1;
                }
            },
        }
        self.last_direction = self.direction;

        // check if we hit ourselves
        if self.get(self.head[0], self.head[1]) > 0 {
            return TickResult::Lose("Ouroboros".to_owned())
        }

        // check if we hit food
        if self.get(self.head[0], self.head[1]) < 0 {
            self.size += 1;
            if let Err(str) = self.new_food() {
                if str == "No space for food" {
                    return TickResult::Win("Yummers".to_owned())
                }
                return TickResult::Lose("Garbage".to_owned())
            }
        } else {
            // decay snake
            for i in 0..(self.width*self.height).into() {
                if self.table[i] > 0 {
                    self.table[i] -= 1;
                }
            }
        }

        TickResult::Continue
    }

    pub fn draw(&mut self) {
        vdp::clear_color(vdp::Color32::new(0, 0, 0, 255));
        vdp::clear_depth(1.0);

        let mut tris = Vec::<vdp::Vertex>::new();

        let size = 1.0 / (max(self.width, self.height) as f32);

        for x in 0..self.width {
            for y in 0..self.height {
                match self.at(x, y) {
                    Location::Head(_) => {
                        draw::floor_box(&mut tris, x as f32, y as f32, 0.0, size, Vector4::new(0.15, 0.15, 0.15, 1.0));
                        let scale = if self.size == 1 { 0.6 } else if self.size == 2 { 0.6 } else if self.size == 3 { 0.8 } else { 1.0 };
                        draw::body_box(&mut tris, true, x as f32, y as f32, 0.0, size, scale);
                        if x == 0 {
                            let state_floaty = self.tick_state_floaty(self.width as i8, y as i8);
                            draw::body_prediction_box(&mut tris, true, self.width as f32, y as f32, 0.0, size, vec3(0.0, 0.5, 0.0), state_floaty);
                        }
                        if y == 0 {
                            let state_floaty = self.tick_state_floaty(x as i8, self.height as i8);
                            draw::body_prediction_box(&mut tris, true, x as f32, self.height as f32, 0.0, size, vec3(0.5, 0.0, 0.0), state_floaty);
                        }
                        if x == self.width - 1 {
                            let state_floaty = self.tick_state_floaty(-1, y as i8);
                            draw::body_prediction_box(&mut tris, true, -1.0, y as f32, 0.0, size, vec3(1.0, 0.5, 0.0), state_floaty);
                        }
                        if y == self.height - 1 {
                            let state_floaty = self.tick_state_floaty(x as i8, -1);
                            draw::body_prediction_box(&mut tris, true, x as f32, -1.0, 0.0, size, vec3(0.5, 1.0, 0.0), state_floaty);
                        }
                    }
                    Location::Body(val) => {
                        draw::floor_box(&mut tris, x as f32, y as f32, 0.0, size, Vector4::new(0.15, 0.15, 0.15, 1.0));
                        let scale: f32 = if val == 1 { 0.4 } else if val == 2 { 0.65 } else if val == 3 { 0.9 } else { 1.0 };
                        draw::body_box(&mut tris, false, x as f32, y as f32, 0.0, size, scale);
                        if x == 0 {
                            let state_floaty = self.tick_state_floaty(self.width as i8, y as i8);
                            draw::body_prediction_box(&mut tris, false, self.width as f32, y as f32, 0.0, size, vec3(0.0, 0.5, 0.0), state_floaty);
                        }
                        if y == 0 {
                            let state_floaty = self.tick_state_floaty(x as i8, self.height as i8);
                            draw::body_prediction_box(&mut tris, false, x as f32, self.height as f32, 0.0, size, vec3(0.5, 0.0, 0.0), state_floaty);
                        }
                        if x == self.width - 1 {
                            let state_floaty = self.tick_state_floaty(-1, y as i8);
                            draw::body_prediction_box(&mut tris, false, -1.0, y as f32, 0.0, size, vec3(1.0, 0.5, 0.0), state_floaty);
                        }
                        if y == self.height - 1 {
                            let state_floaty = self.tick_state_floaty(x as i8, -1);
                            draw::body_prediction_box(&mut tris, false, x as f32, -1.0, 0.0, size, vec3(0.5, 1.0, 0.0), state_floaty);
                        }
                    }
                    Location::Food => {
                        draw::floor_box(&mut tris, x as f32, y as f32, 0.0, size, Vector4::new(0.15, 0.15, 0.15, 1.0));
                        let state_floaty = self.tick_state_floaty(x as i8, y as i8);
                        draw::food_box(&mut tris, x as f32, y as f32, 0.0, size, state_floaty);
                    }
                    Location::Empty => {
                        draw::floor_box(&mut tris, x as f32, y as f32, 0.0, size, Vector4::new(0.25, 0.25, 0.25, 1.0));
                    }
                }
            }
        }
        
        let cam_offsets = self.floaty_camera.offsets((self.tick as f32) / (self.interval_frames as f32));
        draw::transform_draw_tris(&mut tris, cam_offsets)
    }
}
