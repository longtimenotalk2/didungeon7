use crate::game701::common::{Dir, Id};

use super::{Pose, UnitMut};

impl<'a> UnitMut<'a> {
    // in round
    pub fn refresh_active(&mut self) {
        self.board.unit_data_mut(self.id).is_active = true;
    }

    pub fn end_active(&mut self) {
        self.board.unit_data_mut(self.id).is_active = false;
    }

    // act
    pub fn dash_to_unit(&mut self, id : Id) -> Option<Dir> {
        let pos_self = self.board.get_pos(self.id);
        let pos = self.board.get_pos(id);
        if pos_self < pos {
            self.board.unit_move(self.id, pos-1);
            Some(Dir::Right)
        } else if pos_self > pos {
            self.board.unit_move(self.id, pos+1);
            Some(Dir::Left)
        } else {
            None
        }
    }

    pub fn set_dir(&mut self, dir : Dir) {
        if self.immute_core().is_stand() {
            self.board.unit_data_mut(self.id).pose = Pose::from_dir(dir);
        }
    }

    // be act

    pub fn take_dmg(&mut self, dmg : i32) {
        self.board.unit_data_mut(self.id).hp -= dmg;
        if self.board.unit_data(self.id).hp <= 0 {
            self.board.unit_data_mut(self.id).hp = 0;
        }
    }

    pub fn take_tie(&mut self, rope : i32) {
        macro_rules! unit_mut {
            () => {
                self.board.unit_data_mut(self.id)
            };
        }
        unit_mut!().pose = Pose::Fall;

        unit_mut!().bound_upper += rope;
        let overflow = if unit_mut!().bound_upper > 4 {
            unit_mut!().bound_upper - 4
        } else {0};
        unit_mut!().bound_upper -= overflow;

        unit_mut!().bound_lower += overflow;
        let overflow = if unit_mut!().bound_lower > 4 {
            unit_mut!().bound_lower - 4
        } else {0};
        unit_mut!().bound_lower -= overflow;
    }

    pub fn be_pinned_from(&mut self, dir_atk : Dir) {
        macro_rules! unit_ref {
            () => {
                self.board.unit_data(self.id)
            };
        }

        macro_rules! unit {
            () => {
                self.immute_core()
            };
        }

        macro_rules! unit_mut {
            () => {
                self.board.unit_data_mut(self.id)
            };
        }

        match unit_ref!().pose {
            Pose::Alert => {unit_mut!().pose = Pose::from_dir(dir_atk.anti());},
            Pose::Left | Pose::Right => if unit!().is_backstab_from(dir_atk) && unit!().is_sandwich_from(dir_atk) {
                unit_mut!().pose = Pose::Confuse;
            },
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game701::board::Board;

    #[test]
    fn test_dash_to_unit() {
        let mut board = Board::new_team();
        board.unit_mut(5).dash_to_unit(0);
        board.show();
    }
}