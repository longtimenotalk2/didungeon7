use super::Unit;

impl<'a> Unit<'a> {
    pub fn get_name(&self) -> &str {
        &self.unit_data().name
    }
}