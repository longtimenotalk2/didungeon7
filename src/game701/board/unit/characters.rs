use super::{Team, UnitData};

impl UnitData {
    pub fn new_noal() -> Self {
        Self::new(
            "诺艾尔",
            Team::Ally,
            32, // max_hp,
            8, // basic_melee_atk
            9, // basic_melee_def
            10, // basic_acc
            12, // basic_evd
            7, // basic_cri
            13,  // basic_lck
            10, // basic_spd
            5, // rope_tie
            2, // rope_struggle
            4, // rope_rescue
        )
    }

    pub fn new_yelin() -> Self {
        Self::new(
            "叶  琳",
            Team::Ally,
            50, // max_hp,
            13, // basic_melee_atk
            13, // basic_melee_def
            13, // basic_acc
            12, // basic_evd
            13, // basic_cri
            5,  // basic_lck
            11, // basic_spd
            3, // rope_tie
            1, // rope_struggle
            2, // rope_rescue
        )
    }

    pub fn new_alyssa() -> Self {
        Self::new(
            "艾丽莎",
            Team::Ally,
            38, // max_hp,
            9, // basic_melee_atk
            9, // basic_melee_def
            11, // basic_acc
            17, // basic_evd
            13, // basic_cri
            12,  // basic_lck
            16, // basic_spd
            4, // rope_tie
            2, // rope_struggle
            3, // rope_rescue
        )
    }

    pub fn new_elis() -> Self {
        Self::new(
            "伊莉丝",
            Team::Ally,
            42, // max_hp,
            10, // basic_melee_atk
            11, // basic_melee_def
            16, // basic_acc
            12, // basic_evd
            13, // basic_cri
            11,  // basic_lck
            13, // basic_spd
            4, // rope_tie
            2, // rope_struggle
            3, // rope_rescue
        )
    }
}