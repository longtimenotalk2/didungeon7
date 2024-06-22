use miniserde::{Deserialize, Serialize};
use rand::Rng;

use crate::game701::common::Dir;

use super::super::common::Id;

use super::Board;

mod attribute;
mod new;
mod fmt;
mod characters;
mod enemys;
mod action;
mod scan;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
enum Team {
    Ally,
    Enemy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
enum Pose {
    Alert,
    Left,
    Right,
    Confuse,
    Fall,
}

impl Pose {
    pub fn from_dir(dir : Dir) -> Pose {
        match dir {
            Dir::Left => Pose::Left,
            Dir::Right => Pose::Right,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UnitData {
    name : String,
    team : Team,

    // attribute
    max_hp : i32,
    basic_melee_atk : i32,
    basic_melee_def : i32,
    basic_acc : i32,
    basic_evd : i32,
    basic_cri : i32,
    basic_lck : i32,
    basic_spd : i32,
    rope_tie : i32,
    rope_struggle : i32,
    rope_rescue : i32,

    // state
    is_active : bool,
    pose : Pose,
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

    pub fn immute_core(&self) -> Unit {
        Unit::create(&self.board, self.id)
    }

    pub fn other(&self, id : Id) -> Unit {
        Unit::create(&self.board, id)
    }

    pub fn other_mut(&mut self, id : Id) -> UnitMut {
        UnitMut::create(self.board, id)
    }

    pub fn d100(&mut self) -> i32 {
        self.board.rng().gen_range(1..=100)
    }
}