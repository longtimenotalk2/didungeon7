use super::super::common::Id;

use super::Board;

mod attribute;
mod new;
mod fmt;
mod characters;
mod enemys;
mod action;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Team {
    Ally,
    Enemy,
}

pub struct UnitData {
    name : String,
    team : Team,

    // attribute
    max_hp : i32,
    basic_melee_atk : i32,
    basic_melee_def : i32,
    basic_spd : i32,
    rope_tie : i32,
    rope_struggle : i32,
    rope_rescue : i32,

    // state
    is_active : bool,
    hp : i32,
    bound_upper : i32,
    bound_lower : i32,
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
        self.board.unit_data(self.id)
    }
}

pub struct UnitMut<'a> {
    board : &'a mut Board,
    id : Id,
}

impl<'a> UnitMut<'a> {
    pub fn create(board : &'a mut Board, id : Id) -> UnitMut<'a> {
        Self {
            board,
            id,
        }
    }

    fn immute_core(&self) -> Unit {
        Unit::create(&self.board, self.id)
    }
}