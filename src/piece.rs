use crate::shape::{Blocks, Shape};

/// A movable piece.
#[derive(Clone, Copy)]
pub struct Piece {
    /// The shape.
    shape: Shape,

    /// The X position.
    x: i8,

    /// The Y position.
    y: i8,
}

impl Piece {
    /// Create a new piece from its shape and position.
    pub fn new(shape: Shape, x: i8, y: i8) -> Self {
        Self { shape, x, y }
    }

    /// Get the shape.
    pub fn shape(self) -> Shape {
        self.shape
    }

    /// Get the blocks.
    pub fn blocks(self) -> Blocks {
        self.shape.blocks().map(|(x, y)| (x + self.x, y + self.y))
    }
}
