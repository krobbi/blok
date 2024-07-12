mod blocks;

pub use blocks::Blocks;

use crate::engine::{tileset, Engine};

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

    /// The bounding width of an upward-facing shape.
    pub const WIDTH: usize = 4;

    /// The bounding height of an upward-facing shape.
    pub const HEIGHT: usize = 2;

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

    /// Get the ghost tile.
    pub fn ghost_tile(self) -> usize {
        match self {
            Self::I => tileset::CYAN_GHOST_TILE,
            Self::J => tileset::BLUE_GHOST_TILE,
            Self::L => tileset::ORANGE_GHOST_TILE,
            Self::O => tileset::YELLOW_GHOST_TILE,
            Self::S => tileset::GREEN_GHOST_TILE,
            Self::T => tileset::PURPLE_GHOST_TILE,
            Self::Z => tileset::RED_GHOST_TILE,
        }
    }

    /// Get the blocks with a facing.
    pub fn blocks(self, facing: Facing) -> Blocks {
        blocks::new(self, facing)
    }

    /// Draw the shape.
    pub fn draw(self, x: usize, y: usize, engine: &mut Engine) {
        for y in y..y + Self::HEIGHT {
            for x in x..x + Self::WIDTH {
                engine.draw_tile(tileset::CLEAR_TILE, x, y);
            }
        }

        let tile = self.block_tile();

        for (base_x, base_y) in self.blocks(Facing::Up) {
            #[allow(clippy::cast_sign_loss)]
            let (x, y) = (base_x as usize + x, base_y as usize + y);

            engine.draw_tile(tile, x, y);
        }
    }
}
