use crate::game701::{board::unit::UnitMut, common::Id};

impl<'a> UnitMut<'a> {
    pub fn tie(&mut self, id : Id) {
        let dir = self.dash_to_unit(id);
        if let Some(d) = dir {
            self.set_dir(d);
        }
        let rope = self.immute_core().rope_tie();
        self.other_mut(id).take_tie(rope);
    }

    pub fn struggle(&mut self) {
        let rope = self.immute_core().rope_struggle();
        self.take_struggle(rope);
    }
    

    pub fn rescue(&mut self, id : Id) {
        let dir = self.dash_to_unit(id);
        if let Some(d) = dir {
            self.set_dir(d);
        }
        let rope = self.immute_core().rope_rescue();
        self.other_mut(id).take_rescue(rope);
    }
}