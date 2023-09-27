mod snake {

    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    pub struct Game {
        Width: u8,
        Height: u8,
        Table: Vec<u32>,
        
        Size: u32,
        LastDirection: Direction,
        Direction: Direction,
        Head: [u8; 2],
    }

    impl Game {
        pub fn new(width: u8, height: u8) -> Game {
            Game {
                Width: width,
                Height: height,
                Table: Vec::new(),
                
                Size: 0,
                LastDirection: Direction::Right,
                Direction: Direction::Right,
                Head: [0, 0],
            }
        }
    }

}
