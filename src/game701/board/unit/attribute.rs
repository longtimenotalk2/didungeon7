use super::{Team, Unit};

impl<'a> Unit<'a> {
    // basic
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

    pub fn team(&self) -> Team {
        self.unit_data().team
    }

    // ability
    pub fn max_hp(&self) -> i32 {
        self.unit_data().max_hp
    }

    pub fn hp(&self) -> i32 {
        self.unit_data().hp
    }

    pub fn spd_original(&self) -> i32 {
        self.unit_data().basic_spd
    }

    pub fn spd(&self) -> i32 {
        self.spd_original() + self.board.spd_fixs[self.id]
    }

    pub fn order_point(&self) -> i32 {
        self.spd() * 10 + self.id as i32
    }

    // state
    pub fn is_active(&self) -> bool {
        self.unit_data().is_active
    }
}