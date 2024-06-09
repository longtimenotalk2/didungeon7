use super::Unit;

impl<'a> Unit<'a> {
    pub fn name(&self) -> &str {
        &self.unit_data().name
    }

    pub fn max_hp(&self) -> i32 {
        self.unit_data().max_hp
    }

    pub fn hp(&self) -> i32 {
        self.unit_data().hp
    }
}
