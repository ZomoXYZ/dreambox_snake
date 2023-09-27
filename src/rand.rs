use dbsdk_rs::clock;

static MAX: u128 = 0xFFFFFFFF;

pub struct Rng {
    seed: u128,
}

impl Rng {
    pub fn new() -> Rng {
        Rng {
            seed: seed(),
        }
    }

    pub fn random(&mut self, max: u8) -> u8 {
        let seed = self.seed.pow(2) % MAX;
        self.seed = seed;
        (seed % (max as u128)) as u8
    }
}

pub fn seed() -> u128 {
    let time = clock::get_time();
    // (time.second as u128) + (time.minute as u128) * 60 + (time.hour as u128) * 60 * 60 + (time.day as u128) * 60 * 60 * 24 + (time.month as u128) * 60 * 60 * 24 * 30 + (time.year as u128) * 60 * 60 * 24 * 30 * 12
    ((time.second + time.minute + time.hour + time.day + time.month) as u128 + 26713) % MAX
}
