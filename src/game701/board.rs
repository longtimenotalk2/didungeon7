use unit::{Unit, UnitData};

use super::common::Id;

pub mod unit;

pub struct Board {
    units : Vec<UnitData>,
}

impl Board {
    pub fn new() -> Board {
        Self {
            units : Vec::new(),
        }
    }

    pub fn add_unit(&mut self, unit_data : UnitData) {
        self.units.push(unit_data);
    }

    pub fn unit(&self, id: Id) -> Unit {
        Unit::create(&self, id)
    }

    fn unit_data_with_id(&self, id : Id) -> &UnitData {
        &self.units[id]
    }
}