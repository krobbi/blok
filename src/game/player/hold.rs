use crate::engine::Engine;

use super::{Board, Shape};

/// A container holding an optional shape.
pub struct Hold {
    /// The optional held shape.
    shape: Option<Shape>,

    /// Whether the hold is unlocked.
    unlocked: bool,
}

impl Hold {
    /// The width for drawing a hold in tiles.
    pub const WIDTH: usize = 1 + Shape::WIDTH + 1;

    /// The height for drawing a hold in tiles.
    pub const HEIGHT: usize = 1 + Shape::HEIGHT + 1;

    /// Create a new hold.
    pub fn new() -> Self {
        let shape = None;
        let unlocked = true;
        Self { shape, unlocked }
    }

    /// Reset the hold.
    pub fn reset(&mut self) {
        self.shape = None;
        self.unlocked = true;
    }

    /// Get whether the hold is unlocked.
    pub fn unlocked(&self) -> bool {
        self.unlocked
    }

    /// Hold a shape in the hold and get the optional previously held shape.
    pub fn hold(&mut self, shape: Shape) -> Option<Shape> {
        self.unlocked = false;
        let previous_shape = self.shape;
        self.shape = Some(shape);
        previous_shape
    }

    /// Unlock the hold.
    pub fn unlock(&mut self) {
        self.unlocked = true;
    }

    /// Draw the hold.
    pub fn draw(&self, engine: &mut Engine) {
        const X: usize = Board::HOLD_X + 1;
        const Y: usize = Board::HOLD_Y + 1;

        engine.draw_border(Board::HOLD_X, Board::HOLD_Y, Self::WIDTH, Self::HEIGHT);
        Shape::draw_background(X, Y, engine);

        if let Some(shape) = self.shape {
            shape.draw(X, Y, engine);
        }
    }
}
