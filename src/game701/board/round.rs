use rand::Rng;

use super::Board;

impl Board{
    pub fn round_start(&mut self) {
        self.round += 1;
        // active everyone
        for id in 0..self.len() {
            self.unit_mut(id).refresh_active()
        }
        // random change spd
        for f in self.spd_fixs.iter_mut() {
            *f = self.rng.gen_range(-4..=4);
        }
    }


}