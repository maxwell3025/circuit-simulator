#[derive(Clone, Copy)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}

impl Direction{
    pub fn left(&self) -> Direction{
        match self{
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
    pub fn right(&self) -> Direction{
        match self{
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }
}