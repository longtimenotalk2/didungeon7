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
}

#[cfg(test)]
mod test {
    use crate::game701::board::Board;

    #[test]
    fn test_scan_touch_stand_enemy() {
        let board = Board::new_team();
        let unit = board.unit(0);
        dbg!(unit.scan_touch_stand_enemy(1));
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
        let mut block_acc = 0;
        let mut c = |end : i32| {
            for pos in pos_self..=end {
                if pos != pos_self {
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
        c(0);
        c((self.board.len() - 1) as Pos);
        
        pos_scan
    }
}

#[cfg(test)]
mod test_scan {
    use crate::game701::board::Board;

    #[test]
    fn test_scan() {
        let board = Board::new_team();
        let unit = board.unit(0);
        dbg!(unit.scan());
    }
}