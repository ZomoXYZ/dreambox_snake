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

note
    sm64's rng implementation will return 2 u8 numbers, which is why `next` and `random` return [u8; 2]
    the second number can be discarded if you only need 1 number
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

    // 2 u8 numbers
    pub fn next(&mut self) -> [u8; 2] {
        let mut num: [u8; 2] = [0, 0];

        self.tick();
        num[0] = self.seeds[0] ^ self.seeds[1];
        self.tick();
        num[1] = self.seeds[0] ^ self.seeds[1];

        num
    }

    // 2 u8 numbers between 0 and max
    pub fn random(&mut self, max: u8) -> [u8; 2] {
        let mut num = self.next();

        num[0] = num[0] % max;
        num[1] = num[1] % max;

        num
    }

    // combine 2 u8 numbers into 1 u16 number between 0 and max
    pub fn random_single(&mut self, max: u16) -> u16 {
        let num = self.next();
        (((num[0] as u16) << 8) | num[1] as u16) % max
    }

    // combine 4 u8 numbers into 1 u32 number between 0 and max
    pub fn random_single_u32(&mut self, max: u32) -> u32 {
        let num0 = self.next();
        let num1 = self.next();
        (((num0[0] as u32) << 24) | ((num0[1] as u32) << 16) | ((num1[0] as u32) << 8) | num1[1] as u32) % max
    }

    fn tick(&mut self) {
        self.seeds[0] = self.seeds[0].wrapping_mul(5).wrapping_add(1);

        let bit4 = self.seeds[1] & 8 == 8;
        let bit7 = self.seeds[1] & 64 == 64;
        let odd = if bit4 == bit7 { 1 } else { 0 };
        
        self.seeds[1] = self.seeds[1].wrapping_mul(2).wrapping_add(odd)
    }
}
