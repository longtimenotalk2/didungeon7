pub const DEBUG_GLOBAL: bool = false;

pub type Id = usize;
pub type Pos = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
}

impl Dir {
    pub fn anti(&self) -> Self {
        match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    pub fn next_pos(&self, start : Pos) -> Pos {
        match self {
            Dir::Left => start - 1,
            Dir::Right => start + 1,
        }
    }
}