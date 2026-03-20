use strum::EnumIter;

use crate::tiles::Tile;

/// A shape.
#[derive(Clone, Copy, EnumIter)]
pub enum Shape {
    /// An 'I' shape.
    I,

    /// A 'J' shape.
    J,

    /// An 'L' shape.
    L,

    /// An 'O' shape.
    O,

    /// An 'S' shape.
    S,

    /// A 'T' shape.
    T,

    /// A 'Z' shape.
    Z,
}

impl Shape {
    /// Returns the `Shape`'s block [`Tile`].
    pub const fn block_tile(self) -> Tile {
        match self {
            Self::I => Tile::CyanBlock,
            Self::J => Tile::BlueBlock,
            Self::L => Tile::OrangeBlock,
            Self::O => Tile::YellowBlock,
            Self::S => Tile::GreenBlock,
            Self::T => Tile::PurpleBlock,
            Self::Z => Tile::RedBlock,
        }
    }
}
