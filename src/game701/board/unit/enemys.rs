use super::{Team, UnitData};

impl UnitData {
    pub fn new_fighter() -> Self {
        Self::new(
            "女战士",
            Team::Enemy,
            50, // max_hp,
            10, // basic_melee_atk
            10, // basic_melee_def
            10, // basic_spd
            3, // rope_tie
            1, // rope_struggle
            2, // rope_rescue
        )
    }

    pub fn new_thief() -> Self {
        Self::new(
            "女盗贼",
            Team::Enemy,
            40, // max_hp,
            8, // basic_melee_atk
            9, // basic_melee_def
            13, // basic_spd
            4, // rope_tie
            1, // rope_struggle
            3, // rope_rescue
        )
    }
}