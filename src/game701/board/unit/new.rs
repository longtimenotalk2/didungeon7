use super::UnitData;

impl UnitData {
    pub fn new(
        name : &str,
    ) -> Self {
        Self {
            name : name.to_string(),
        }
    }
}