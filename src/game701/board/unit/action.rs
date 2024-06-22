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

    pub fn auto_heal(&mut self) {
        let heal = self.immute_core().max_hp() / 10;
        self.take_heal(heal);
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

    pub fn try_stand_and_alert(&mut self) {
        if self.immute_core().can_stand() {
            self.board.unit_data_mut(self.id).pose = Pose::Alert;
        }
    }

    // be act

    pub fn take_dmg(&mut self, dmg : i32) {
        self.board.unit_data_mut(self.id).hp -= dmg;
        if self.board.unit_data(self.id).hp <= 0 {
            self.board.unit_data_mut(self.id).hp = 0;
        }
    }

    pub fn take_heal(&mut self, heal : i32) {
        self.board.unit_data_mut(self.id).hp += heal;
        if self.immute_core().hp() > self.immute_core().max_hp() {
            self.board.unit_data_mut(self.id).hp = self.immute_core().max_hp();
        }
    }

    pub fn fall_down(&mut self) {
        self.board.unit_data_mut(self.id).pose = Pose::Fall;
    }

    pub fn take_tie(&mut self, rope : i32) {
        macro_rules! unit_ref {
            () => {
                self.board.unit_data(self.id)
            };
        }
        macro_rules! unit_mut {
            () => {
                self.board.unit_data_mut(self.id)
            };
        }
        self.fall_down();

        let mut upper = unit_ref!().bound_upper;
        let mut lower = unit_ref!().bound_lower;
        const BOUND_MAX : i32 = 4;

        // 优先捆绑上肢
        let upper_add = BOUND_MAX - upper;
        if rope <= upper_add {
            unit_mut!().bound_upper += rope;
        } else {
            unit_mut!().bound_upper = BOUND_MAX;
            let rope = rope - upper_add;
            let lower_add = BOUND_MAX - lower;
            if rope <= lower_add {
                unit_mut!().bound_lower += rope;
            } else {
                unit_mut!().bound_lower = BOUND_MAX;
            }
        }
    }

    fn rope_reduce(&mut self, rope : i32, can_reduce_when_max : bool) {
        macro_rules! unit_ref {
            () => {
                self.board.unit_data(self.id)
            };
        }
        macro_rules! unit_mut {
            () => {
                self.board.unit_data_mut(self.id)
            };
        }
        // 1. 上下肢全满则挣扎失败
        // 2. 优先挣扎层数少的，如果一样则优先挣扎下肢

        const BOUND_MAX : i32 = 4;
        
        for _ in 0..rope {
            let upper = unit_ref!().bound_upper;
            let lower = unit_ref!().bound_lower;
            if upper == 0 {
                if !lower == 0 {
                    unit_mut!().bound_lower -= 1;
                }
            } else {
                if lower == 0 {
                    unit_mut!().bound_upper -= 1;
                } else {
                    if lower < upper {
                        unit_mut!().bound_lower -= 1;
                    } else if lower > upper {
                        unit_mut!().bound_upper -= 1;
                    } else {
                        if lower != BOUND_MAX || can_reduce_when_max {
                            unit_mut!().bound_lower -= 1;
                        }
                    }
                }
            }
        }
    }

    pub fn take_struggle(&mut self, rope : i32) {
        self.rope_reduce(rope, false);
    }

    pub fn take_rescue(&mut self, rope : i32) {
        self.rope_reduce(rope, true);
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