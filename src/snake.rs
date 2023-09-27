use std::cmp::min;
use dbsdk_rs::vdp::{self, Color32};

use rand;
use draw;

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
    
    pub size: u16,
    last_direction: Direction,
    direction: Direction,
    pub head: [u8; 2],

    interval_frames: u32,
    interval_frame: u32,
    frame: u32,
    tick: u32,

    rng: rand::Rng,
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
        let mut table = Vec::with_capacity((width * height).into());
        for _ in 0..width*height {
            table.push(0);
        }


        let mut game = Game {
            width,
            height,
            table,
            
            size: 1,
            last_direction: Direction::Right,
            direction: Direction::Right,
            head: [left-1, height-top], // should this be rng?

            interval_frames,
            interval_frame: 0,
            frame: 0,
            tick: 0,

            rng: rand::Rng::new(),
            last_tick: TickResult::Continue,
        };
        let _ = game.new_food(); // rng will be consistent if i call it here
        return game
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
        let x = self.rng.random(self.width);
        let y = self.rng.random(self.height);

        let index = self.get_index(x, y);
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
        vdp::clear_color(Color32::new(0, 0, 0, 255));
        vdp::clear_depth(1.0);

        let mut tris = Vec::<vdp::Vertex>::new();

        let size = 1.0 / (min(self.width, self.height) as f32);

        for x in 0..self.width {
            for y in 0..self.height {
                match self.at(x, y) {
                    Location::Head(_) => {
                        draw::head_box(&mut tris, x, y, size)
                    }
                    Location::Body(_) => {
                        draw::body_box(&mut tris, x, y, size)
                    }
                    Location::Food => {
                        draw::food_box(&mut tris, x, y, size);
                    }
                    Location::Empty => {
                        draw::empty_box(&mut tris, x, y, size);
                    }
                }
            }
        }

        vdp::draw_geometry(vdp::Topology::TriangleList, &tris);
    }
}
