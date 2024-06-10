use rand::Rng;

use crate::game701::common::Id;

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

    pub fn round_main(&mut self) -> bool {
        // find id with active and most order_point
        let mut pool = vec![];
        for id in 0..self.len() {
            let unit = self.unit(id);
            if unit.is_active() {
                pool.push((id, unit.order_point()));
            }
        }
        if pool.is_empty() {
            return false;
        }
        pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let id = pool[0].0;

        // id's turn
        self.turn(id);

        // end active
        self.unit_mut(id).end_active();

        true
    }

    pub fn turn(&mut self, id : Id) {

    }
}