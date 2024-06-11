use crate::game701::{board::unit::UnitMut, common::Id};

impl<'a> UnitMut<'a> {
    pub fn combat_touch(&mut self, id : Id) {
        self.cause_dmg(id);
        self.dash_to_unit(id);
    }

    fn cause_dmg(&mut self, id : Id) {
        macro_rules! tar {
            () => {
                self.other(id)
            };
        }
        let atk = self.immute_core().melee_atk();
        let def = tar!().melee_def();
        
    }
}

fn dmg_basic(atk : i32, def : i32) -> f64 {
    let atk = atk as f64;
    let def = def as f64;
    let dmg = atk * ((100. - def) / 100.);
    dmg
}