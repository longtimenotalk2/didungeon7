use colorful::{Color, Colorful};

use crate::game701::{common::{Id, Pos}, skill::Skill};

use super::{Pose, Team, Unit};

impl<'a> Unit<'a> {
    // basic
    pub fn name_original(&self) -> &str {
        &self.unit_data().name
    }

    pub fn name(&self) -> String {
        let mut name = self.unit_data().name.clone();
        if *self.board.name_manager.get(&name).unwrap() == 0 {
            name += " ";
        } else {
            name += &self.board.name_adder[self.id];
        }
        name
    }

    pub fn colored_name(&self) -> String {
        let name_color = match self.team() {
            Team::Ally => Color::Blue,
            Team::Enemy => Color::Red,
        };
        format!("{}", self.name().color(name_color))
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn pos(&self) -> Pos {
        self.board.get_pos(self.id)
    }

    pub fn team(&self) -> Team {
        self.unit_data().team
    }

    // ability
    pub fn max_hp(&self) -> i32 {
        self.unit_data().max_hp
    }

    pub fn hp(&self) -> i32 {
        self.unit_data().hp
    }

    pub fn melee_atk(&self) -> i32 {
        self.unit_data().basic_melee_atk
    }

    pub fn melee_def(&self) -> i32 {
        self.unit_data().basic_melee_def
    }

    pub fn spd_original(&self) -> i32 {
        self.unit_data().basic_spd
    }

    pub fn spd(&self) -> i32 {
        self.spd_original() + self.board.spd_fixs[self.id]
    }

    pub fn rope_tie(&self) -> i32 {
        self.unit_data().rope_tie
    }

    pub fn order_point(&self) -> i32 {
        self.spd() * 10 + self.id as i32
    }

    pub fn skills(&self) -> Vec<Skill> {
        Skill::basic_set()
    }

    // state
    pub fn is_active(&self) -> bool {
        self.unit_data().is_active
    }

    pub fn pose(&self) -> Pose {
        self.unit_data().pose
    }

    pub fn is_stand(&self) -> bool {
        self.unit_data().pose != Pose::Fall
    }

    pub fn bound_upper(&self) -> i32 {
        self.unit_data().bound_upper
    }

    pub fn bound_lower(&self) -> i32 {
        self.unit_data().bound_lower
    }

    pub fn arm_can_use(&self) -> bool {
        self.unit_data().bound_upper == 0
    }

    pub fn is_weak(&self) -> bool {
        self.hp() as f64 / self.max_hp() as f64 <= 0.2
    }

    // interaction
    pub fn can_block(&self) -> bool {
        self.is_stand()
    }
}
