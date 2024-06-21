use super::{unit::UnitData, Board};

impl Board {
    pub fn start() -> Self {
        match Self::load_default() {
            Ok(board) => board,
            Err(_) => Self::new_team(),
        }
    }

    pub fn new_team() -> Self {
        let mut board = Board::new(114514);
        board.add_unit(UnitData::new_noal());
        board.add_unit(UnitData::new_elis());
        board.add_unit(UnitData::new_alyssa());
        board.add_unit(UnitData::new_yelin());
        board.add_unit(UnitData::new_fighter());
        board.add_unit(UnitData::new_fighter());
        board.add_unit(UnitData::new_thief());
        board.add_unit(UnitData::new_thief());

        board
    }
}