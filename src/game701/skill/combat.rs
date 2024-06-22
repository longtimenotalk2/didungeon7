use colorful::{Color, Colorful};

use crate::game701::{board::unit::UnitMut, common::{Dir, Id}};

impl<'a> UnitMut<'a> {
    pub fn combat_touch(&mut self, id : Id, acc_skill : i32, cri_skill : i32) {
        let dir_atk = self.dash_to_unit(id);
        if let Some(d) = dir_atk {
            self.set_dir(d);
        }
        self.cause_dmg(id, dir_atk, acc_skill, cri_skill);
    }

    fn cause_dmg(&mut self, id : Id, dir_atk : Option<Dir>, acc_skill : i32, cri_skill : i32) {
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
        let is_backstab = dir_atk.is_some_and(|d| tar!().is_backstab_from(d));
        let is_sandwich = dir_atk.is_some_and(|d| tar!().is_sandwich_from(d));

        let hit = hit_rate(acc, evd, acc_skill, is_backstab, is_sandwich);
        let is_hit = hit >= self.d100();
        let crit = cri_rate(cri, lck, cri_skill, is_backstab, is_sandwich);
        let is_crit = crit >= self.d100();

        let base_dmg = dmg_basic(atk, def, is_backstab, is_sandwich);
        let dmg = match is_crit {
            true => (base_dmg * (150 + self.d100()) as f64 / 50 as f64) as i32,
            false => (base_dmg * (100 + self.d100()) as f64 / 100 as f64) as i32,
        };
        let base_info = format!("hit = {}, cri = {}{}{}",
            hit,
            crit,
            if is_backstab {", 背刺"} else {""},
            if is_sandwich {", 夹击"} else {""},
        );
        if is_hit {
            tar_mut!().take_dmg(dmg);
            if let Some(d) = dir_atk {
                tar_mut!().be_pinned_from(d);
            }
            println!("{} 对 {} 造成 【 {} 】 点伤害{}    ({})" , self.immute_core().colored_name(), tar!().colored_name(), 
                dmg.to_string().color(Color::Yellow), 
                match is_crit {
                    true => "（暴击！）",
                    false => "",
                },
                base_info
            );
            println!()
        } else {
            println!("{} 未命中 {}    ({})", self.immute_core().colored_name(), tar!().colored_name(),
                base_info,
            )
        }
        println!()
    }
}

fn cri_rate(cri : i32, lck : i32, basic_cri : i32, is_backtrab : bool, is_sandwich : bool) -> i32 {
    let mut crit = basic_cri;
    if is_backtrab {crit += 20};
    if is_sandwich {crit += 20};
    (crit + (cri - lck)).min(100).max(0)
}

fn hit_rate(acc : i32, evd : i32, acc_skill : i32, is_backtrab : bool, is_sandwich : bool) -> i32 {
    let mut acc_sum = acc_skill;
    if is_backtrab {acc_sum += 20};
    if is_sandwich {acc_sum += 20};
    (acc_sum + 2 * (acc - evd)).min(95).max(5)
}

fn dmg_basic(atk : i32, def : i32, is_backtrab : bool, is_sandwich : bool) -> f64 {
    let mut atk = atk as f64;
    let mut def = def as f64;
    if is_backtrab {atk *= 1.2; def *= 0.8};
    if is_sandwich {atk *= 1.2; def *= 0.8};
    let dmg = atk * ((100. - def) / 100.);
    dmg
}