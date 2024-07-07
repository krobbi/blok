use crate::engine::{tileset, Engine};

use super::{piece::Piece, shape::Shape};

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

    /// The height of a buffer above a board in cells.
    const BUFFER_HEIGHT: usize = 3;

    /// The capacity of a board in cells.
    const CAPACITY: usize = (Self::HEIGHT + Self::BUFFER_HEIGHT) * Self::WIDTH;

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

    /// Clear the board.
    pub fn clear(&mut self) {
        self.cells.fill(None);
    }

    /// Create a new optional piece from its shape if it would fit on the board.
    pub fn create_piece(&self, shape: Shape) -> Option<Piece> {
        #[allow(clippy::cast_possible_truncation)]
        let mut piece = Piece::new(shape, (Board::WIDTH as i8 - 4) / 2, -2);

        if self.fits_piece(piece, 0, 0) {
            piece.drop(self);
            Some(piece)
        } else {
            None
        }
    }

    /// Get whether the board can fit a piece with an offset.
    pub fn fits_piece(&self, piece: Piece, x: i8, y: i8) -> bool {
        for (base_x, base_y) in piece.blocks() {
            let (x, y) = (base_x + x, base_y + y);

            #[allow(clippy::cast_possible_truncation)]
            if x < 0
                || x >= Self::WIDTH as i8
                || y >= Self::HEIGHT as i8
                || y >= -(Self::BUFFER_HEIGHT as i8) && self.cells[Self::cell_index(x, y)].is_some()
            {
                return false;
            }
        }

        true
    }

    /// Attempt to lock a piece on the board and get whether it was successful.
    pub fn lock_piece(&mut self, piece: Piece) -> bool {
        if Self::piece_above_board(piece) {
            return false;
        }

        let cell = Some(piece.shape());

        for (x, y) in piece.blocks() {
            self.cells[Self::cell_index(x, y)] = cell;
        }

        true
    }

    /// Clear all full lines on the board.
    pub fn clear_lines(&mut self) {
        for index in (0..Self::CAPACITY).step_by(Self::WIDTH) {
            if !self.cells[index..index + Self::WIDTH].contains(&None) {
                self.cells.copy_within(0..index, Self::WIDTH);
                self.cells[0..Self::WIDTH].fill(None);
            }
        }
    }

    /// Draw the board.
    pub fn draw(&self, engine: &mut Engine) {
        engine.draw_border(Self::DRAW_X, Self::DRAW_Y, Self::WIDTH, Self::HEIGHT);

        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                let tile = match self.cells[(y + Self::BUFFER_HEIGHT) * Self::WIDTH + x] {
                    None => tileset::CELL_TILE,
                    Some(shape) => shape.block_tile(),
                };

                let (x, y) = (x + Self::DRAW_X, y + Self::DRAW_Y);
                engine.draw_tile(tile, x, y);
            }
        }
    }

    /// Get a cell index from a block position.
    fn cell_index(x: i8, y: i8) -> usize {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        {
            (y + Self::BUFFER_HEIGHT as i8) as usize * Self::WIDTH + x as usize
        }
    }

    /// Get whether a piece is entirely above a board.
    fn piece_above_board(piece: Piece) -> bool {
        for (_, y) in piece.blocks() {
            if y >= 0 {
                return false;
            }
        }

        true
    }
}
