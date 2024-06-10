use std::collections::HashMap;

use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use unit::{Unit, UnitData, UnitMut};

use super::common::{Id, Pos};

pub mod unit;
pub mod new;

mod operate;
mod fmt;
mod round;

pub struct Board {
    // basic
    units : Vec<UnitData>,
    ids_by_position : Vec<Id>,
    // round
    round : i32,
    spd_fixs : Vec<i32>,
    // other
    name_manager : HashMap<String, i32>,
    name_adder : Vec<&'static str>,
    rng : ChaChaRng,
}

impl Board {
    pub fn new(seed : u64) -> Board {
        Self {
            units : Vec::new(),
            ids_by_position : Vec::new(),
            round : 0,
            spd_fixs : Vec::new(),
            name_manager : HashMap::new(),
            name_adder : Vec::new(),
            rng : ChaChaRng::seed_from_u64(seed),
        }
    }

    pub fn add_unit(&mut self, unit_data : UnitData) {
        self.units.push(unit_data);
        self.ids_by_position.push(self.units.len() - 1);
        self.spd_fixs.push(0);

        let name = self.unit(self.units.len()-1).name_original().to_string();
        self.name_manager.entry(name.clone()).and_modify(|counter| *counter += 1).or_insert(0);
        let counter = *self.name_manager.get(&name).unwrap();

        fn usize2letter(i: usize) -> &'static str {
            // transform integer to letter ABCDE...
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ".get(i..i+1).unwrap()
          }
        self.name_adder.push(usize2letter(counter as usize));

    }

    pub fn len(&self) -> usize {
        self.units.len()
    }

    pub fn unit(&self, id: Id) -> Unit {
        Unit::create(&self, id)
    }

    pub fn unit_mut(&mut self, id: Id) -> UnitMut {
        UnitMut::create(self, id)
    }

    fn unit_data(&self, id : Id) -> &UnitData {
        &self.units[id]
    }

    fn unit_data_mut(&mut self, id : Id) -> &mut UnitData {
        &mut self.units[id]
    }

    fn get_pos(&self, id : Id) -> Pos {
        for (p, i) in self.ids_by_position.iter().enumerate() {
            if *i == id {
                return p as i32;
            }
        }
        unimplemented!()
    }

    fn get_id_from_pos(&self, pos: Pos) -> Id {
        let p : usize = pos.try_into().unwrap();
        self.ids_by_position[p]
    }

}