use super::{board::{unit::{Unit, UnitMut}, Board}, common::Id};

mod combat;

#[derive(Clone)]
pub enum Skill {
    Melee,
    Skip,
}

impl Skill {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Melee => "体术",
            Self::Skip => "略过",
        }
    }

    pub fn basic_set() -> Vec<Self> {
        vec![Self::Melee, Self::Skip]
    }

    fn link(&self) -> SkillData {
        match self {
            Self::Melee => SkillData::Melee,
            Self::Skip => SkillData::Skip,
        }
    }

    pub fn can_use(&self, unit : &Unit) -> bool {
        self.link().can_use(unit)
    }

    pub fn find_targets(&self, unit : &Unit) -> Vec<Target> {
        self.link().find_targets(unit)
    }

    pub fn exe(&self, unit : &mut UnitMut, target : Target) {
        self.link().exe(unit, target)
    }
}

enum SkillData {
    Melee,
    Skip,
}

#[derive(Clone)]
pub enum Target {
    Unit(Id),
}

impl Target {
    pub fn name_in_board(&self, board : &Board) -> String {
        match self {
            Self::Unit(id) => board.unit(*id).name().to_string(),
        }
    }

    pub fn assert_unit(&self) -> Id {
        match self {
            Self::Unit(id) => *id,
            _ => unreachable!(),
        }
    }
}

impl SkillData {
    fn can_use(&self, unit : &Unit) -> bool {
        match self {
            Self::Melee => unit.arm_can_use(),
            Self::Skip => true,
        }
    }

    fn find_targets(&self, unit : &Unit) -> Vec<Target> {
        fn ids_fmt(l : Vec<Id>) -> Vec<Target> {
            l.iter().map(|id| Target::Unit(*id)).collect()
        }

        let to_self = || {vec![Target::Unit(unit.id())]};

        match self {
            Self::Melee => ids_fmt(unit.scan_touch_stand_enemy(1)),
            Self::Skip => to_self(),
        }
    }

    fn exe(&self, unit : &mut UnitMut, target : Target) {
        match self {
            Self::Melee => unit.combat_touch(target.assert_unit()),
            Self::Skip => (),
        }
    }
}