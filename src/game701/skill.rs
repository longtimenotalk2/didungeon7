use miniserde::{Deserialize, Serialize};

use crate::game701::board::unit::scan::TargetDemandCard;

use super::{board::{unit::{Unit, UnitMut}, Board}, common::Id};

mod combat;
mod rope;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Skill {
    Melee,
    Tie,
    Struggle,
    Rescue,
    Skip,
}

impl Skill {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Melee => "体术",
            Self::Tie => "捆绑",
            Self::Struggle => "挣扎",
            Self::Rescue => "解绑",
            Self::Skip => "略过",
        }
    }

    pub fn basic_set() -> Vec<Self> {
        vec![
            Self::Melee, 
            Self::Tie,
            Self::Struggle,
            Self::Rescue,
            Self::Skip,
        ]
    }

    fn link(&self) -> SkillData {
        match self {
            Self::Melee => SkillData::Melee,
            Self::Tie => SkillData::Tie,
            Self::Struggle => SkillData::Struggle,
            Self::Rescue => SkillData::Rescue,
            Self::Skip => SkillData::Skip,
        }
    }

    pub fn can_use(&self, unit : Unit) -> bool {
        self.link().can_use(unit)
    }

    pub fn find_targets(&self, unit : Unit) -> Vec<Target> {
        self.link().find_targets(unit)
    }

    pub fn exe(&self, unit : UnitMut, target : Target) {
        self.link().exe(unit, target)
    }
}

enum SkillData {
    Melee,
    Tie,
    Struggle,
    Rescue,
    Skip,
}

#[derive(Clone, Debug)]
pub enum Target {
    Unit(Id),
}

impl Target {
    pub fn new_unit(id : Id) -> Self {
        Self::Unit(id)
    }

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
    fn can_use(&self, unit : Unit) -> bool {
        match self {
            Self::Melee => unit.arm_can_use() && unit.is_stand(),
            Self::Tie => unit.arm_can_use() && unit.is_stand(),
            Self::Struggle => unit.has_bound(),
            Self::Rescue => unit.arm_can_use() && unit.is_stand(),
            Self::Skip => true,
        }
    }

    fn find_targets(&self, unit : Unit) -> Vec<Target> {
        fn ids_fmt(l : Vec<Id>) -> Vec<Target> {
            l.iter().map(|id| Target::Unit(*id)).collect()
        }

        let to_self = || {vec![Target::new_unit(unit.id())]};

        match self {
            Self::Melee => ids_fmt(unit.scan_main(TargetDemandCard::new_enemy().only_stand().set_bypass(1))),
            Self::Tie => ids_fmt(unit.scan_main(TargetDemandCard::new_enemy().only_weak().or_fall())),
            Self::Struggle => to_self(),
            Self::Rescue => ids_fmt(unit.scan_main(TargetDemandCard::new_ally().only_bound())),
            Self::Skip => to_self(),
        }
    }

    fn exe(&self, mut unit : UnitMut, target : Target) {
        match self {
            Self::Melee => unit.combat_touch(target.assert_unit(), 85, 10),
            Self::Tie => unit.tie(target.assert_unit()),
            Self::Struggle => unit.struggle(),
            Self::Rescue => unit.rescue(target.assert_unit()),
            Self::Skip => (),
        }
    }
}