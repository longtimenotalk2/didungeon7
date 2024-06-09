use super::{Team, Unit};

impl<'a> Unit<'a> {
    pub fn name_original(&self) -> &str {
        &self.unit_data().name
    }

    pub fn name(&self) -> String {
        let mut name = self.unit_data().name.clone();
        if *self.board.name_manager.get(&name).unwrap() == 0 {
            name += " ";
        } else {
            name += self.board.name_adder[self.id];
        }
        name
    }

    pub fn max_hp(&self) -> i32 {
        self.unit_data().max_hp
    }

    pub fn hp(&self) -> i32 {
        self.unit_data().hp
    }

    pub fn team(&self) -> Team {
        self.unit_data().team
    }
}
