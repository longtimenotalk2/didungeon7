use super::Board;

impl Board {
    pub fn show(&self) {
        for id in &self.ids_by_position {
            self.unit(*id).show();
            println!();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game701::board::{unit::UnitData, Board};

    #[test]
    fn test_show() {
        let mut board = Board::new(114514);
        board.add_unit(UnitData::new_noal());
        board.add_unit(UnitData::new_elis());
        board.add_unit(UnitData::new_alyssa());
        board.add_unit(UnitData::new_yelin());
        board.add_unit(UnitData::new_fighter());
        board.add_unit(UnitData::new_fighter());
        board.add_unit(UnitData::new_thief());
        board.add_unit(UnitData::new_thief());

        board.show();
    }
}