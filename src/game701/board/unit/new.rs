use super::UnitData;

impl UnitData {
    // pub fn new(
    //     name : &str,
    //     max_hp : i32,
    //     basic_spd : i32,
    // ) -> Self {
    //     Self {
    //         name : name.to_string(),
    //         max_hp,
    //         basic_spd,
    //     }
    // }

    pub fn new_default() -> Self {
        Self {
            name : "艾丽莎".to_string(),
            max_hp : 500,
            basic_melee_atk : 100,
            basic_melee_def : 100,
            basic_spd : 100,
            hp : 400,
        }
    }
}