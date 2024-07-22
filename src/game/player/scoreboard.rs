use crate::engine::Engine;

use super::Board;

/// A display counting a player's score.
pub struct Scoreboard {
    /// The score.
    score: usize,
}

impl Scoreboard {
    /// Create a new scoreboard.
    pub fn new() -> Self {
        let score = 0;
        Self { score }
    }

    /// Draw the scoreboard.
    pub fn draw(&self, engine: &mut Engine) {
        engine.draw_number(self.score, Board::SCOREBOARD_X, Board::SCOREBOARD_Y);
    }
}
