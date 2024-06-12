use crate::game701::{board::unit::UnitMut, common::Id};

impl<'a> UnitMut<'a> {
    pub fn tie(&mut self, id : Id) {
        self.dash_to_unit(id);
        let rope = self.immute_core().rope_tie();
        self.other_mut(id).take_tie(rope);
    }
}