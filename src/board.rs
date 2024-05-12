use std::fmt::Display;

use miniserde::{Deserialize, Serialize};

use crate::unit::Unit;

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    units: Vec<Unit>,
}

impl Board {
    
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for u in &self.units {
            u.fmt(f)?;
        }
        Ok(())
    }
}

