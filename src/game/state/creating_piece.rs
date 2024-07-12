use crate::{
    engine::Engine,
    game::{
        player::{Player, Shape},
        state,
    },
};

use super::{State, Transition};

/// A state where a new piece is created from a shape.
pub struct CreatingPiece {
    /// The shape for creating the piece.
    shape: Shape,
}

impl CreatingPiece {
    /// Create a new creating piece state from its shape.
    pub fn new(shape: Shape) -> Self {
        Self { shape }
    }
}

impl State for CreatingPiece {
    fn update(&mut self, player: &mut Player, _engine: &Engine) -> Transition {
        match player.board().create_piece(self.shape) {
            None => state::GameOver.transition(),
            Some(piece) => state::DroppingPiece::new(piece).transition(),
        }
    }
}
