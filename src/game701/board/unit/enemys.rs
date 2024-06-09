use super::{Team, UnitData};

impl UnitData {
    pub fn new_fighter() -> Self {
        Self::new(
            "女战士",
            Team::Enemy,
            500, // max_hp,
            100, // basic_melee_atk
            100, // basic_melee_def
            100, // basic_spd
            3, // rope_tie
            1, // rope_struggle
            2, // rope_rescue
        )
    }

    pub fn new_thief() -> Self {
        Self::new(
            "女盗贼",
            Team::Enemy,
            400, // max_hp,
            85, // basic_melee_atk
            90, // basic_melee_def
            110, // basic_spd
            4, // rope_tie
            1, // rope_struggle
            3, // rope_rescue
        )
    }
}