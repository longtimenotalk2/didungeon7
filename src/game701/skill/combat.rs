use crate::game701::{board::unit::UnitMut, common::Id};

impl<'a> UnitMut<'a> {
    pub fn combat_touch(&mut self, id : Id) {
        self.dash_to_unit(id);
        self.cause_dmg(id);
    }

    fn cause_dmg(&mut self, id : Id) {
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
        let dmg = dmg_basic(atk, def) as i32;
        tar_mut!().take_dmg(dmg);
    }
}

fn dmg_basic(atk : i32, def : i32) -> f64 {
    let atk = atk as f64;
    let def = def as f64;
    let dmg = atk * ((100. - def) / 100.);
    dmg
}