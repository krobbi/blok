/// A direction faced by a piece.
#[derive(Clone, Copy)]
pub enum Facing {
    /// An upward direction faced by a piece.
    Up,

    /// A rightward direction faced by a piece.
    Right,

    /// A downward direction faced by a piece.
    Down,

    /// A leftward direction faced by a piece.
    Left,
}

impl Facing {
    /// The number of facings.
    pub const COUNT: usize = 4;

    /// Get the next facing clockwise.
    pub fn clockwise_facing(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}
