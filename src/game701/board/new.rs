use std::io;

use super::{unit::UnitData, Board};

impl Board {
    pub fn start() -> Self {
        match Self::load_default() {
            Ok(board) => {
                println!("按回车读档，输入任意字符重开");
                let mut ops = String::new();
                io::stdin().read_line(&mut ops).expect("failed to read line");
                if ops.trim() == "" {
                    board
                } else {
                    Self::new_team()
                }
            },
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

    pub fn test() -> Self {
        let mut board = Board::new(114514);
        board.add_unit(UnitData::new_noal());
        board.add_unit(UnitData::new_alyssa());
        board.add_unit(UnitData::new_fighter());
        board.add_unit(UnitData::new_thief());

        board
    }
}