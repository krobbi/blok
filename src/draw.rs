use crate::{
    canvas::Canvas,
    tiles::{Tile, Tileset},
};

/// Context for drawing.
pub struct DrawContext<'app> {
    /// The [`Tileset`].
    tileset: &'app Tileset,

    /// The frame buffer as a slice of pixel components in RGBA order.
    buffer: &'app mut [u8],
}

impl<'app> DrawContext<'app> {
    /// The horizontal resolution in [`Tile`]s.
    pub const TILES_ACROSS: usize = Canvas::WIDTH / Tile::WIDTH;

    /// The vertical resolution in [`Tile`]s.
    pub const TILES_DOWN: usize = Canvas::HEIGHT / Tile::HEIGHT;

    /// Creates a new `DrawContext` from a [`Tileset`] and a frame buffer.
    pub const fn new(tileset: &'app Tileset, buffer: &'app mut [u8]) -> Self {
        Self { tileset, buffer }
    }

    /// Draws a [`Tile`] at a position.
    pub fn draw_tile(&mut self, tile: Tile, x: u8, y: u8) {
        const OFFSET_X: usize = (Canvas::WIDTH - DrawContext::TILES_ACROSS * Tile::WIDTH) / 2;
        const OFFSET_Y: usize = (Canvas::HEIGHT - DrawContext::TILES_DOWN * Tile::HEIGHT) / 2;
        const STRIP_LENGTH: usize = Tile::WIDTH * Canvas::CHANNEL_COUNT;
        const LINE_LENGTH: usize = Canvas::WIDTH * Canvas::CHANNEL_COUNT;

        let mut buffer_index = (usize::from(y) * Tile::HEIGHT + OFFSET_Y) * LINE_LENGTH
            + (usize::from(x) * Tile::WIDTH + OFFSET_X) * Canvas::CHANNEL_COUNT;

        for strip in self.tileset.tile_graphic(tile).chunks_exact(STRIP_LENGTH) {
            self.buffer[buffer_index..buffer_index + STRIP_LENGTH].copy_from_slice(strip);
            buffer_index += LINE_LENGTH;
        }
    }
}
