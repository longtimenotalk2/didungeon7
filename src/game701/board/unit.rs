use super::super::common::Id;

use super::Board;

mod basic;
mod new;
mod fmt;

pub struct UnitData {
    name : String,
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
        self.board.get_unit_data_with_id(self.id)
    }
}