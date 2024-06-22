use colorful::{Color, Colorful};

use crate::game701::{board::OrderValue, common::{Dir, Id, Pos}, skill::Skill};

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

    pub fn is_hp_empty(&self) -> bool {
        self.hp() <= 0
    }

    pub fn melee_atk(&self) -> i32 {
        let mut v = self.unit_data().basic_melee_atk as f64;
        if self.is_weak() {
            v *= 0.8;
        }
        v.ceil() as i32
    }

    pub fn melee_def(&self) -> i32 {
        let mut v = self.unit_data().basic_melee_def as f64;
        if self.is_weak() {
            v *= 0.8;
        }
        v.ceil() as i32
    }

    pub fn acc(&self) -> i32 {
        let mut v = self.unit_data().basic_evd as f64;
        if self.is_weak() {
            v *= 0.8;
        }
        v.ceil() as i32
    }

    pub fn evd(&self) -> i32 {
        let mut v = self.unit_data().basic_evd as f64;
        if self.is_weak() {
            v *= 0.8;
        }
        v.ceil() as i32
    }

    pub fn cri(&self) -> i32 {
        let mut v = self.unit_data().basic_cri as f64;
        if self.is_weak() {
            v *= 0.8;
        }
        v.ceil() as i32
    }

    pub fn lck(&self) -> i32 {
        let mut v = self.unit_data().basic_lck as f64;
        if self.is_weak() {
            v *= 0.8;
        }
        v.ceil() as i32
    }

    pub fn spd(&self) -> i32 {
        let mut v = self.unit_data().basic_spd as f64;
        if self.is_weak() {
            v *= 0.8;
        }
        v.ceil() as i32
    }

    pub(in super::super) fn order_value(&self) -> OrderValue {
        OrderValue::new(
            self.id(), 
            self.spd(), 
            self.board.spd_fixs[self.id()], 
            self.is_stand(),
        )
    }

    pub fn rope_tie(&self) -> i32 {
        self.unit_data().rope_tie
    }

    pub fn rope_struggle(&self) -> i32 {
        self.unit_data().rope_struggle
    }

    pub fn rope_rescue(&self) -> i32 {
        self.unit_data().rope_rescue
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

    pub fn has_bound(&self) -> bool {
        self.unit_data().bound_upper!= 0 || self.unit_data().bound_lower!= 0
    }

    pub fn is_weak(&self) -> bool {
        self.hp() as f64 / self.max_hp() as f64 <= 0.2
    }

    // cans
    pub fn arm_can_use(&self) -> bool {
        self.unit_data().bound_upper == 0
    }

    pub fn leg_can_use(&self) -> bool {
        self.unit_data().bound_lower == 0
    }

    pub fn can_stand(&self) -> bool {
        self.arm_can_use() | self.leg_can_use()
    }

    pub fn can_block(&self) -> bool {
        self.is_stand()
    }

    // about pose
    pub fn is_backstab_from(&self, dir_atk : Dir) -> bool {
        match self.unit_data().pose {
            Pose::Alert => false,
            Pose::Left => dir_atk == Dir::Left,
            Pose::Right => dir_atk == Dir::Right,
            Pose::Confuse => true,
            Pose::Fall => false,
        }
    }

    pub fn can_pin_with_dir(&self, dir : Dir) -> bool {
        match self.unit_data().pose {
            Pose::Alert => true,
            Pose::Left => dir == Dir::Left,
            Pose::Right => dir == Dir::Right,
            Pose::Confuse => false,
            Pose::Fall => false,
        }
    }

    pub fn is_sandwich_from(&self, dir_atk : Dir) -> bool {
        let pos_pinner = dir_atk.next_pos(self.pos());
        if self.board.pos_is_valid(pos_pinner) {
            let pinner = self.board.unit(self.board.get_id_from_pos(pos_pinner));
            if 
                pinner.team() != self.team() &&
                pinner.can_pin_with_dir(dir_atk.anti())
            {
                return true;
            }
        }
        false
    }
}
