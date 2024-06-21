use colorful::{Color, Colorful};

use crate::game701::{board::unit::UnitMut, common::Id};

impl<'a> UnitMut<'a> {
    pub fn combat_touch(&mut self, id : Id, acc_skill : i32, cri_skill : i32) {
        self.dash_to_unit(id);
        self.cause_dmg(id, acc_skill, cri_skill);
    }

    fn cause_dmg(&mut self, id : Id, acc_skill : i32, cri_skill : i32) {
        macro_rules! tar {
            () => {
                self.other(id)
            };
        }
        macro_rules! tar_mut {
            () => {
                self.other_mut(id)
            };
        }
        let atk = self.immute_core().melee_atk();
        let def = tar!().melee_def();
        let acc = self.immute_core().acc();
        let evd = tar!().evd();
        let cri = self.immute_core().cri();
        let lck = tar!().lck();

        let hit = hit_rate(acc, evd, acc_skill);
        let is_hit = hit >= self.d100();
        let crit = cri_rate(cri, lck, cri_skill);
        let is_crit = crit >= self.d100();

        let dmg = match is_crit {
            true => (dmg_basic(atk, def) * (150 + self.d100()) as f64 / 100 as f64) as i32,
            false => (dmg_basic(atk, def) * (100 + self.d100()) as f64 / 200 as f64) as i32,
        };
        if is_hit {
            tar_mut!().take_dmg(dmg);
            println!("{} 对 {} 造成 【 {} 】 点伤害{}    (hit = {}, cri = {})" , self.immute_core().colored_name(), tar!().colored_name(), 
                dmg.to_string().color(Color::Yellow), 
                match is_crit {
                    true => "（暴击！）",
                    false => "",
                },
                hit,
                crit,
            );
            println!()
        } else {
            println!("{} 未命中 {}    (hit = {}, cri = {})", self.immute_core().colored_name(), tar!().colored_name(),
                hit,
                crit,
            )
        }
        println!()
    }
}

fn cri_rate(cri : i32, lck : i32, basic_cri : i32) -> i32 {
    (basic_cri + (cri - lck)).min(100).max(0)
}

fn hit_rate(acc : i32, evd : i32, acc_skill : i32) -> i32 {
    (acc_skill + 2 * (acc - evd)).min(95).max(5)
}

fn dmg_basic(atk : i32, def : i32) -> f64 {
    let atk = atk as f64;
    let def = def as f64;
    let dmg = atk * ((100. - def) / 100.);
    dmg
}