mod facing;

pub use facing::Facing;

use crate::shape::{Blocks, Shape};

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

    /// Rotate the piece clockwise.
    pub fn rotate_clockwise(&mut self) {
        self.rotate_to(self.facing.clockwise_facing());
    }

    /// Rotate the piece to a target facing.
    fn rotate_to(&mut self, facing: Facing) {
        self.facing = facing;
    }
}
