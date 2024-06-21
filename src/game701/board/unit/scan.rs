use std::collections::HashMap;

use crate::game701::common::{Id, Pos};

use super::Unit;

impl<'a> Unit<'a> {
    pub fn scan_touch_stand_enemy(&self, bypass : i32) -> Vec<Id> {
        let mut targets = vec![];
        let team = self.team();
        for (id_tar, scan) in self.scan() {
            let tar = self.board.unit(id_tar);
            if 
                tar.team() != team // enemy
                && tar.is_stand() // stand
                && scan.block_num <= bypass // bypass limit
            {
                targets.push(id_tar);
            }
        }
        targets.sort();
        targets
    }

    pub fn scan_touch_weak_or_fall_enemy(&self, bypass : i32) -> Vec<Id> {
        let mut targets = vec![];
        let team = self.team();
        for (id_tar, scan) in self.scan() {
            let tar = self.board.unit(id_tar);
            if 
                tar.team() != team // enemy
                && (tar.is_weak() || !tar.is_stand()) // weak or fall
                && scan.block_num <= bypass // bypass limit
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