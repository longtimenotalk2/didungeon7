use rand::Rng;

use crate::game701::{common::Id, skill::Target};

use super::Board;

pub mod io;
mod file;

impl Board{
    pub fn play(&mut self) {
        for _ in 0..100 {
            let has_active_unit = self.round_main();
            if !has_active_unit {
                self.round_next();
            }
        }
    }

    fn round_next(&mut self) {
        self.round += 1;
        println!("================第 {} 轮================", self.round);
        
        for id in 0..self.len() {
            // active everyone
            self.unit_mut(id).refresh_active();
            // auto heal
            self.unit_mut(id).auto_heal();
            // auto stand
            self.unit_mut(id).try_stand_and_alert();
        }
        // random change spd
        let mut rng = self.rng();
        for f in self.spd_fixs.iter_mut() {
            *f = rng.gen_range(-4..=4);
        }
    }

    pub fn round_main(&mut self) -> bool {
        // auto save
        self.save_default();
        
        // find id with active and most order_point
        let mut pool = vec![];
        for id in 0..self.len() {
            let unit = self.unit(id);
            if unit.is_active() {
                pool.push((id, unit.order_value()));
            }
        }
        if pool.is_empty() {
            return false;
        }
        pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let id = pool[0].0;

        // id's turn
        self.turn(id);

        // end active
        self.unit_mut(id).end_active();

        true
    }

    pub fn turn(&mut self, id : Id) {
        macro_rules! unit {
            () => {
                self.unit(id)
            };
        }
        macro_rules! unit_mut {
            () => {
                self.unit_mut(id)
            };
        }

        // try stand
        unit_mut!().try_stand_and_alert();

        // show board and title
        self.id_now = Some(id);
        self.show();
        println!();
        println!("{} 的回合：", unit!().colored_name());

        // skill_options
        println!();
        let mut skill_options = vec![];
        let mut skills = vec![];
        for skill in unit!().skills() {
            if skill.can_use(unit!()) {
                let targets = skill.find_targets(unit!());
                if targets.len() > 0 {
                    skill_options.push(skill.name().to_string());
                    skills.push(skill);
                }
            }
        }

        assert!(skills.len() > 0); // has skip

        let select_i = io::io_select_from_list(skill_options);
        let skill = skills[select_i].clone();

        // target_options
        let targets = skill.find_targets(unit!());
        let target_options = targets.iter().map(|t| t.name_in_board(&self)).collect();

        let select_i = io::io_select_from_list(target_options);
        let target = targets[select_i].clone();

        // exe
        skill.exe(unit_mut!(), target);

    }
}