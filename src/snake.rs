use std::time::{Duration, Instant};
use getrandom::getrandom;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    width: u8,
    height: u8,
    table: Vec<i32>,
    
    size: u16,
    last_direction: Direction,
    direction: Direction,
    head: [u8; 2],

    interval_ms: f32,
    last_tick: Instant,
}

enum TickResult<T, E> {
    Win(T),
    Lose(E),
    Continue
}

impl Game {
    pub fn new(width: u8, height: u8, interval_ms: f32) -> Game {
        let mut table = Vec::with_capacity((width * height).into());
        for _ in 0..width*height {
            table.push(0);
        }

        let last_tick = Instant::now();

        Game {
            width,
            height,
            table,
            
            size: 0,
            last_direction: Direction::Right,
            direction: Direction::Right,
            head: [0, 0],

            interval_ms,
            last_tick,
        }
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

    fn get_index(&self, x: u8, y: u8) -> usize {
        if x >= self.width || y >= self.height {
            return 0
        }
        (y as usize) * (self.width as usize) + (x as usize)
    }
    fn get(&self, x: u8, y: u8) -> i32 {
        self.table[self.get_index(x, y)]
    }
    fn set(&mut self, x: u8, y: u8, value: i32) {
        let index = self.get_index(x, y);
        self.table[index] = value;
    }

    fn new_food(&mut self) -> Result<(), &str> {
        let mut pos: [u8; 2] = [0, 0];
        if let Err(s) = getrandom(&mut pos) {
            return Err("Failed to get random")
        }

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

    fn tick(&mut self) -> TickResult<&str, &str> {
        if self.last_tick.elapsed() < Duration::from_millis(self.interval_ms as u64) {
            return TickResult::Continue
        }

        // set head
        self.set(self.head[0], self.head[1], self.size.into());

        // move head
        match self.direction {
            Direction::Up => {
                if self.head[1] == 0 {
                    self.head[1] = self.height - 1;
                } else {
                    self.head[1] -= 1;
                }
            },
            Direction::Down => {
                if self.head[1] == self.height - 1 {
                    self.head[1] = 0;
                } else {
                    self.head[1] += 1;
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
        self.last_tick = Instant::now();

        // check if we hit ourselves
        if self.get(self.head[0], self.head[1]) > 0 {
            return TickResult::Lose("Ouroboros")
        }

        // check if we hit food
        if self.get(self.head[0], self.head[1]) == 0 {
            self.size += 1;
            if let Err(str) = self.new_food() {
                if str == "No space for food" {
                    return TickResult::Win("Yummers")
                }
                return TickResult::Lose("Garbage")
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
}
