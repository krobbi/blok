mod facing;

pub use facing::Facing;

use crate::{
    board::Board,
    shape::{Blocks, Shape},
};

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

    /// Drop the piece by one cell if it would fit on a board.
    pub fn drop(&mut self, board: &Board) {
        if board.fits_piece(*self, 0, 1) {
            self.y += 1;
        }
    }

    /// Move the piece up by one cell if it would fit on a board.
    // TODO: Remove this method.
    pub fn test_move_up(&mut self, board: &Board) {
        if self.y > -4 && board.fits_piece(*self, 0, -1) {
            self.y -= 1;
        }
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
        let source_facing = self.facing;
        self.facing = facing;

        if !board.fits_piece(*self, 0, 0) {
            self.facing = source_facing;
        }
    }
}
