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

    /// Draws a border around a rectangle.
    pub fn draw_border(&mut self, x: usize, y: usize, width: usize, height: usize) {
        let left_x = x - 1;
        let right_x = x + width;
        let top_y = y - 1;
        let bottom_y = y + height;

        self.draw_tile(Tile::TopLeftBorder, left_x, top_y);
        self.draw_tile(Tile::TopRightBorder, right_x, top_y);
        self.draw_tile(Tile::BottomLeftBorder, left_x, bottom_y);
        self.draw_tile(Tile::BottomRightBorder, right_x, bottom_y);

        for x in x..right_x {
            self.draw_tile(Tile::TopBorder, x, top_y);
            self.draw_tile(Tile::BottomBorder, x, bottom_y);
        }

        for y in y..bottom_y {
            self.draw_tile(Tile::LeftBorder, left_x, y);
            self.draw_tile(Tile::RightBorder, right_x, y);
        }
    }

    /// Draws a [`Tile`] at a position.
    pub fn draw_tile(&mut self, tile: Tile, x: usize, y: usize) {
        const X_OFFSET: usize = (Frame::WIDTH - DrawContext::TILES_ACROSS * Tile::WIDTH) / 2;
        const Y_OFFSET: usize = (Frame::HEIGHT - DrawContext::TILES_DOWN * Tile::HEIGHT) / 2;

        let mut buffer_index =
            (y * Tile::HEIGHT + Y_OFFSET) * Frame::WIDTH + x * Tile::WIDTH + X_OFFSET;

        for tile_row in self.tileset.tile_pixels(tile).chunks_exact(Tile::WIDTH) {
            self.buffer[buffer_index..buffer_index + Tile::WIDTH].copy_from_slice(tile_row);
            buffer_index += Frame::WIDTH;
        }
    }
}
