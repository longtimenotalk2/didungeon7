use super::super::common::Id;

use super::Board;

mod basic;
mod new;
mod fmt;

pub struct UnitData {
    name : String,

    // attribute
    max_hp : i32,
    basic_melee_atk : i32,
    basic_melee_def : i32,
    basic_spd : i32,

    // state
    hp : i32,
}
pub struct Unit<'a> {
    board : &'a Board,
    id : Id,
}

impl<'a> Unit<'a> {
    pub fn create(board : &'a Board, id : Id) -> Unit<'a> {
        Self {
            board,
            id,
        }
    }

    fn unit_data(&self) -> &UnitData {
        self.board.unit_data_with_id(self.id)
    }
}