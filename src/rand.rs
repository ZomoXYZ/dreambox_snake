pub struct Rng {
    seed: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Rng {
        Rng {
            seed,
        }
    }

    pub fn random(&self, max: u8) -> u8 {
        let mut seed = self.seed * self.seed * self.seed;
        (seed % (max as u32)) as u8
    }
}
