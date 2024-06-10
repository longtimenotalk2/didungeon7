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

    fn get_target(&self, unit : &Unit) -> Vec<Target> {
        fn ids_fmt(l : Vec<Id>) -> Vec<Target> {
            l.iter().map(|id| Target::Unit(*id)).collect()
        }

        let to_self = || {vec![Target::Unit(unit.id())]};

        match self {
            Self::Melee => ids_fmt(unit.scan_touch_stand_enemy(1)),
            Self::Skip => to_self(),
        }
    }
}