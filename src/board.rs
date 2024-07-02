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
        let mut cells = [None; Self::CAPACITY];

        // Place cells to test collision.
        // TODO: Remove these lines and make cells immutable.
        cells[53] = Some(Shape::Z);
        cells[146] = Some(Shape::S);

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

    /// Get whether the board can fit a piece with an offset.
    pub fn fits_piece(&self, piece: Piece, x: i8, y: i8) -> bool {
        for (base_x, base_y) in piece.blocks() {
            let (x, y) = (base_x + x, base_y + y);

            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_sign_loss)]
            if x < 0
                || x >= Self::WIDTH as i8
                || y >= Self::HEIGHT as i8
                || y >= 0 && self.cells[y as usize * Self::WIDTH + x as usize].is_some()
            {
                return false;
            }
        }

        true
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
