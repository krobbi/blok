mod facing;

pub use facing::Facing;

use super::{shape::Blocks, Board, Shape};

/// A movable piece.
#[derive(Clone, Copy)]
pub struct Piece {
    /// The shape.
    shape: Shape,

    /// The facing.
    facing: Facing,

    /// The X position.
    x: i8,

    /// The Y position.
    y: i8,
}

impl Piece {
    /// Create a new piece from its shape and position.
    pub fn new(shape: Shape, x: i8, y: i8) -> Self {
        let facing = Facing::Up;

        Self {
            shape,
            facing,
            x,
            y,
        }
    }

    /// Get the shape.
    pub fn shape(self) -> Shape {
        self.shape
    }

    /// Get the blocks.
    pub fn blocks(self) -> Blocks {
        self.shape
            .blocks(self.facing)
            .map(|(x, y)| (x + self.x, y + self.y))
    }

    /// Get whether the piece is airborne on a board.
    pub fn airborne(self, board: &Board) -> bool {
        board.fits_piece(self, 0, 1)
    }

    /// Drop the piece by one cell if it is airborne on a board.
    pub fn drop(&mut self, board: &Board) {
        if self.airborne(board) {
            self.y += 1;
        }
    }

    /// Drop the piece until it lands on a board and get how far it dropped.
    pub fn hard_drop(&mut self, board: &Board) -> usize {
        let mut cells = 0;

        while self.airborne(board) {
            self.y += 1;
            cells += 1;
        }

        cells
    }

    /// Move the piece left by one cell if it would fit on a board.
    pub fn move_left(&mut self, board: &Board) {
        if board.fits_piece(*self, -1, 0) {
            self.x -= 1;
        }
    }

    /// Move the piece right by one cell if it would fit on a board.
    pub fn move_right(&mut self, board: &Board) {
        if board.fits_piece(*self, 1, 0) {
            self.x += 1;
        }
    }

    /// Rotate the piece clockwise if it would fit on a board.
    pub fn rotate_clockwise(&mut self, board: &Board) {
        self.rotate_to(self.facing.clockwise_facing(), board);
    }

    /// Rotate the piece counter-clockwise if it would fit on a board.
    pub fn rotate_counter_clockwise(&mut self, board: &Board) {
        self.rotate_to(self.facing.counter_clockwise_facing(), board);
    }

    /// Rotate the piece to a target facing if it would fit on a board.
    fn rotate_to(&mut self, facing: Facing, board: &Board) {
        static DEFAULT_KICKS: &[(i8, i8)] = &[(1, 0), (-1, 0), (0, -1)];
        static I_KICKS: &[(i8, i8)] = &[(1, 0), (-1, 0), (0, -1), (2, 0), (-2, 0), (0, -2)];
        static O_KICKS: &[(i8, i8)] = &[];

        let source_facing = self.facing;
        self.facing = facing;

        if board.fits_piece(*self, 0, 0) {
            return;
        }

        let kicks = match self.shape {
            Shape::I => I_KICKS,
            Shape::J | Shape::L | Shape::S | Shape::T | Shape::Z => DEFAULT_KICKS,
            Shape::O => O_KICKS,
        };

        for &(x, y) in kicks {
            if board.fits_piece(*self, x, y) {
                self.x += x;
                self.y += y;
                return;
            }
        }

        self.facing = source_facing;
    }
}
