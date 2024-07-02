use crate::{
    engine::{tileset, Engine},
    piece::Piece,
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

    /// The X offset for drawing cells.
    const DRAW_X: usize = (Engine::TILES_ACROSS - Self::WIDTH) / 2;

    /// The Y offset for drawing cells.
    const DRAW_Y: usize = (Engine::TILES_DOWN - Self::HEIGHT) / 2;

    /// Create a new board.
    pub fn new() -> Self {
        let cells = [None; Self::CAPACITY];
        Self { cells }
    }

    /// Draw a piece relative to a board.
    pub fn draw_piece(piece: Piece, engine: &mut Engine) {
        let tile = piece.shape().block_tile();

        for (x, y) in piece.blocks() {
            if y >= 0 {
                #[allow(clippy::cast_sign_loss)]
                let (x, y) = (x as usize + Self::DRAW_X, y as usize + Self::DRAW_Y);
                engine.draw_tile(tile, x, y);
            }
        }
    }

    /// Draw the board.
    pub fn draw(&self, engine: &mut Engine) {
        engine.draw_border(Self::DRAW_X, Self::DRAW_Y, Self::WIDTH, Self::HEIGHT);

        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                let tile = match self.cells[y * Self::WIDTH + x] {
                    None => tileset::CELL_TILE,
                    Some(shape) => shape.block_tile(),
                };

                let (x, y) = (x + Self::DRAW_X, y + Self::DRAW_Y);
                engine.draw_tile(tile, x, y);
            }
        }
    }
}
