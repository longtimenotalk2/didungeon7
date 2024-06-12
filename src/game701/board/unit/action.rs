use crate::game701::common::Id;

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
    pub fn dash_to_unit(&mut self, id : Id) {
        let pos_self = self.board.get_pos(self.id);
        let pos = self.board.get_pos(id);
        if pos_self < pos {
            self.board.unit_move(self.id, pos-1);
        } else if pos_self > pos {
            self.board.unit_move(self.id, pos+1);
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