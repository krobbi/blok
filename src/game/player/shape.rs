mod blocks;

pub use blocks::Blocks;

use crate::engine::tileset;

use super::piece::Facing;

/// A piece shape.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// An I shape.
    I,

    /// A J shape.
    J,

    /// An L shape.
    L,

    /// An O shape.
    O,

    /// An S shape.
    S,

    /// A T shape.
    T,

    /// A Z shape.
    Z,
}

impl Shape {
    /// The number of shapes.
    pub const COUNT: usize = 7;

    /// Get the block tile.
    pub fn block_tile(self) -> usize {
        match self {
            Self::I => tileset::CYAN_BLOCK_TILE,
            Self::J => tileset::BLUE_BLOCK_TILE,
            Self::L => tileset::ORANGE_BLOCK_TILE,
            Self::O => tileset::YELLOW_BLOCK_TILE,
            Self::S => tileset::GREEN_BLOCK_TILE,
            Self::T => tileset::PURPLE_BLOCK_TILE,
            Self::Z => tileset::RED_BLOCK_TILE,
        }
    }

    /// Get the blocks with a facing.
    pub fn blocks(self, facing: Facing) -> Blocks {
        blocks::new(self, facing)
    }
}
