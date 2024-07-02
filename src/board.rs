use crate::{
    engine::{tileset, Engine},
    shape::Shape,
};

/// A board containing a grid of cells.
pub struct Board {
    /// The cells.
    cells: [Option<Shape>; Self::CAPACITY],
}

impl Board {
    /// The width of a board in cells.
    const WIDTH: usize = 10;

    /// The height of a board in cells.
    const HEIGHT: usize = 20;

    /// The capacity of a board in cells.
    const CAPACITY: usize = Self::WIDTH * Self::HEIGHT;

    /// Create a new board.
    pub fn new() -> Self {
        let cells = [None; Self::CAPACITY];
        Self { cells }
    }

    /// Draw the board.
    pub fn draw(&self, engine: &mut Engine) {
        const OFFSET_X: usize = (Engine::TILES_ACROSS - Board::WIDTH) / 2;
        const OFFSET_Y: usize = (Engine::TILES_DOWN - Board::HEIGHT) / 2;

        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                let tile = match self.cells[y * Self::WIDTH + x] {
                    None => tileset::CELL_TILE,
                    Some(shape) => shape.block_tile(),
                };

                let (x, y) = (x + OFFSET_X, y + OFFSET_Y);
                engine.draw_tile(tile, x, y);
            }
        }
    }
}
