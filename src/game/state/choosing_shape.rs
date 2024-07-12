use crate::{
    engine::Engine,
    game::{player::Player, state},
};

use super::{State, Transition};

/// A state where a shape for a new piece is chosen after a delay.
pub struct ChoosingShape {
    /// The time until the shape is chosen in seconds.
    delay: f64,
}

impl ChoosingShape {
    /// Create a new choosing shape state.
    pub fn new() -> Self {
        let delay = 0.2;
        Self { delay }
    }
}

impl State for ChoosingShape {
    fn update(&mut self, player: &mut Player, engine: &Engine) -> Transition {
        self.delay -= engine.delta();

        if self.delay <= 0.0 {
            state::CreatingPiece::new(player.queue_mut().shape()).transition()
        } else {
            None
        }
    }
}
