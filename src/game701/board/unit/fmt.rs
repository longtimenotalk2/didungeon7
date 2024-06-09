use super::Unit;

impl<'a> Unit<'a> {
    pub fn show(&self) {
        // name
        print!("{}", self.name());
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
        let id_a = 0;

        macro_rules! unit_a {
            () => {
                board.unit(id_a)
            };
        }

        unit_a!().show();
    }
}