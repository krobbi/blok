use crate::{draw::DrawContext, tiles::Tile};

/// A game world.
pub struct World;

impl World {
    /// Creates a new `World`.
    pub const fn new() -> Self {
        Self
    }

    /// Draws the `World`.
    pub fn draw(&self, draw: &mut DrawContext<'_>) {
        let _ = self;
        draw.draw_tile(Tile::RedBlock, 0, 0);
        draw.draw_tile(Tile::OrangeBlock, 1, 0);
        draw.draw_tile(Tile::YellowBlock, 2, 0);
        draw.draw_tile(Tile::GreenBlock, 0, 1);
        draw.draw_tile(Tile::CyanBlock, 1, 1);
        draw.draw_tile(Tile::BlueBlock, 2, 1);
        draw.draw_tile(Tile::PurpleBlock, 0, 2);
        draw.draw_tile(Tile::Cell, 1, 2);
    }
}
