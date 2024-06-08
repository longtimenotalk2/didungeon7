use super::Unit;

impl<'a> Unit<'a> {
    pub fn show(&self) {
        // name
        print!("{}", self.get_name());
    }
}

#[cfg(test)]
mod test {
    use crate::game701::board::{unit::UnitData, Board};

    #[test]
    fn test_show() {
        let mut board = Board::new();
        let unit_data = UnitData::new("艾丽莎");
        board.add_unit(unit_data);
        let unit = board.get_unit(0);
        unit.show();
    }
}