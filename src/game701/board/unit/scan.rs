use std::{collections::HashMap, default};

use crate::game701::common::{Id, Pos};

use super::Unit;

#[derive(Default)]
pub struct TargetDemandCard {
    is_enemy: bool,
    only_weak : bool,
    or_fall : bool,
    only_stand : bool,
    bypass : i32,
    only_bound : bool,
}

impl TargetDemandCard {
    pub fn new_enemy() -> Self {
        let mut this = Self::default();
        this.is_enemy = true;
        this
    }

    pub fn new_ally() -> Self {
        let mut this = Self::default();
        this
    }

    pub fn only_weak(mut self) -> Self {
        self.only_weak = true;
        self
    }
    
    pub fn set_bypass(mut self, bypass : i32) -> Self {
        self.bypass = bypass;
        self
    }

    pub fn only_stand(mut self) -> Self {
        self.only_stand = true;
        self
    }

    pub fn or_fall(mut self) -> Self {
        self.or_fall = true;
        self
    }

    pub fn only_bound(mut self) -> Self {
        self.only_bound = true;
        self
    }
}

impl<'a> Unit<'a> {
    

    pub fn scan_main(&self, card : TargetDemandCard) -> Vec<Id> {
        let mut targets = vec![];
        let team = self.team();
        for (id_tar, scan) in self.scan() {
            let tar = self.board.unit(id_tar);
            let is_enemy_valid = match card.is_enemy {
                true => tar.team()!= team,
                false => tar.team() == team,
            };
            let only_weak_valid = match card.only_weak {
                true => tar.is_weak(),
                false => true,
            };
            let only_stand_valid = match card.only_stand {
                true => tar.is_stand(),
                false => true,
            };
            let bypass_valid = scan.block_num <= card.bypass;

            let or_fall_valid = match card.or_fall {
                true => !tar.is_stand(),
                false => true,
            };

            let only_bound_valid = match card.only_bound {
                true => tar.has_bound(),
                false => true,
            };

            if 
                is_enemy_valid
                && (only_weak_valid || or_fall_valid)
                && only_stand_valid
                && only_bound_valid
                && bypass_valid
            {
                targets.push(id_tar);
            }
        }
        targets.sort();
        targets
    }
}

#[cfg(test)]
mod test {
    use crate::game701::board::Board;

    #[test]
    fn test_scan_touch_stand_enemy() {
        let board = Board::new_team();
        let unit = board.unit(0);
    }
}

#[derive(Debug)]
struct Scan {
    block_num : i32,
    dist : i32,
}

impl<'a> Unit<'a> {
    fn scan(&self) -> HashMap<Id, Scan> {
        let mut pos_scan : HashMap<Id, Scan> = HashMap::new();
        let team = self.team();
        let pos_self = self.pos();
        let mut c = |dist : i32, is_right : bool| {
            let mut block_acc = 0;
            for d in 0..=dist {
                if d != 0 {
                    let pos = if is_right {pos_self + d} else {pos_self - d};
                    let id_tar = self.board.get_id_from_pos(pos);
                    let tar = self.board.unit(id_tar);
                    pos_scan.insert(id_tar, Scan {
                        block_num : block_acc,
                        dist : d,
                    });
                    if 
                        tar.team() != team
                        && tar.can_block()
                    {
                        block_acc += 1;
                    }
                }
            }
        };
        c(pos_self, false);
        c((self.board.len() - 1) as Pos - pos_self, true);
        
        pos_scan
    }
}

#[cfg(test)]
mod test_scan {
    use crate::game701::{board::Board, common::DEBUG_GLOBAL};

    #[test]
    fn test_scan() {
        let board = Board::new_team();
        let unit = board.unit(0);
    }
}