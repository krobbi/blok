use crate::{
    engine::Engine,
    game::{player::Player, state},
};

use super::{State, Transition};

/// A state where a new piece is attempted to be created on a board with delay.
pub struct CreatingPiece {
    /// The time until the piece is created in seconds.
    delay: f64,
}

impl CreatingPiece {
    /// Create a new creating piece state.
    pub fn new() -> Self {
        let delay = 0.2;
        Self { delay }
    }
}

impl State for CreatingPiece {
    fn update(&mut self, player: &mut Player, engine: &Engine) -> Transition {
        self.delay -= engine.delta();

        if self.delay > 0.0 {
            return None;
        }

        let shape = player.bag_mut().shape();

        match player.board().create_piece(shape) {
            None => state::GameOver.transition(),
            Some(piece) => state::DroppingPiece::new(piece).transition(),
        }
    }
}
