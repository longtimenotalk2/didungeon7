use miniserde::{Deserialize, Serialize};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dice {
    data : u64,
}

impl Dice {
    pub fn new(seed : u64) -> Dice {
        Self {
            data : seed,
        }
    }

    pub fn gen(&mut self) -> ChaChaRng {
        let mut rng = ChaChaRng::seed_from_u64(self.data);
        self.data = rng.gen();
        rng
    }
}