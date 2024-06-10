use super::{Pose, Team, UnitData};

impl UnitData {
    pub fn new(
        name : &str,
        team : Team,
        max_hp : i32,
        basic_melee_atk : i32,
        basic_melee_def : i32,
        basic_spd : i32,
        rope_tie : i32,
        rope_struggle : i32,
        rope_rescue : i32,
    ) -> Self {
        Self {
            name : name.to_string(),
            team,
            max_hp,
            basic_melee_atk,
            basic_melee_def,
            basic_spd,
            rope_tie,
            rope_struggle,
            rope_rescue,
            is_active : false,
            pose : Pose::Alert,
            hp : max_hp,
            bound_upper : 0,
            bound_lower : 0,
        }
    }
}