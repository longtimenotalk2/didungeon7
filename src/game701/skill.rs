use super::{board::unit::Unit, common::Id};

pub enum Skill {
    Melee,
    Skip,
}

enum SkillData {
    Melee,
    Skip,
}

enum Target {
    Unit(Id),
}

impl SkillData {
    fn can_use(&self, unit : &Unit) -> bool {
        match self {
            Self::Melee => unit.arm_can_use(),
            Self::Skip => true,
        }
    }

    // fn get_target(&self, unit : &Unit) -> Vec<Target> {
    //     match self {
    //         Self::Melee => vec![Target::Unit(unit.id)],
    //         Self::Skip => vec![Target::Unit(unit.id())],
    //     }
    // }
}