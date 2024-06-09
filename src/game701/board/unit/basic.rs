use super::Unit;

impl<'a> Unit<'a> {
    pub fn name(&self) -> &str {
        &self.unit_data().name
    }
}