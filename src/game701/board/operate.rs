use crate::game701::common::{Id, Pos};

use super::Board;

impl Board {
    pub fn unit_move(&mut self, id : Id, pos : Pos) {
        self.ids_by_position.remove(self.get_pos(id).try_into().unwrap());
        self.ids_by_position.insert(pos.try_into().unwrap(), id);
    }
}