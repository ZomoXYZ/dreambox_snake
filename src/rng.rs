use dbsdk_rs::clock;

pub fn seeds() -> [u8; 2] {
    let time = clock::get_time();
    [
        time.second + time.minute + time.hour + time.day + time.month,
        (time.second & time.minute) + time.hour + (time.day | time.month)
    ]
}

/*
sm64 rng
https://youtu.be/q15yNrJHOak?t=292
*/
pub struct Rng {
    seeds: [u8; 2]
}

impl Rng {
    pub fn new() -> Rng {
        Rng {
            seeds: seeds(),
        }
    }

    pub fn random(&mut self, max: u8) -> [u8; 2] {
        let mut num: [u8; 2] = [0, 0];

        self.tick();
        num[0] = (self.seeds[0] ^ self.seeds[1]) % max;
        self.tick();
        num[1] = (self.seeds[0] ^ self.seeds[1]) % max;

        num
    }

    fn tick(&mut self) {
        self.seeds[0] = self.seeds[0].wrapping_mul(5).wrapping_add(1);

        let bit4 = self.seeds[1] & 8 == 8;
        let bit7 = self.seeds[1] & 64 == 64;
        let odd = if bit4 == bit7 { 1 } else { 0 };
        
        self.seeds[1] = self.seeds[1].wrapping_mul(2).wrapping_add(odd)
    }
}
