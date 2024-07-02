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
        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                let tile = match self.cells[y * Self::WIDTH + x] {
                    None => tileset::CELL_TILE,
                    Some(shape) => shape.block_tile(),
                };

                engine.draw_tile(tile, x, y);
            }
        }
    }
}
