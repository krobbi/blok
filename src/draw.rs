use crate::{
    frame::Frame,
    tiles::{Tile, Tileset},
};

/// Context for drawing a scene.
pub struct DrawContext<'til, 'buf> {
    /// The [`Tileset`].
    tileset: &'til Tileset,

    /// The frame buffer.
    buffer: &'buf mut [u32; Frame::PIXEL_COUNT],
}

impl<'til, 'buf> DrawContext<'til, 'buf> {
    /// The horizontal [`Tile`] resolution.
    pub const TILES_ACROSS: usize = Frame::WIDTH / Tile::WIDTH;

    /// The vertical [`Tile`] resolution.
    pub const TILES_DOWN: usize = Frame::HEIGHT / Tile::HEIGHT;

    /// Creates a new `DrawContext` from a [`Tileset`] and a [`Frame`].
    pub const fn new(tileset: &'til Tileset, frame: &'buf mut Frame) -> Self {
        Self {
            tileset,
            buffer: frame.as_mut_array(),
        }
    }

    /// Draws a [`Tile`] at a position.
    pub fn draw_tile(&mut self, tile: Tile, x: usize, y: usize) {
        let mut buffer_index = x * Tile::WIDTH + y * Tile::HEIGHT * Frame::WIDTH;

        for tile_row in self.tileset.tile_pixels(tile).chunks_exact(Tile::WIDTH) {
            self.buffer[buffer_index..buffer_index + Tile::WIDTH].copy_from_slice(tile_row);
            buffer_index += Frame::WIDTH;
        }
    }
}
