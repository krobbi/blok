use crate::engine::{tileset, Engine};

use super::Board;

/// A display counting a player's score.
pub struct Scoreboard {
    /// The score.
    score: usize,
}

impl Scoreboard {
    /// The width for drawing a scoreboard in digits.
    pub const WIDTH: usize = 7;

    /// Create a new scoreboard.
    pub fn new() -> Self {
        let score = 0;
        Self { score }
    }

    /// Reset the scoreboard.
    pub fn reset(&mut self) {
        self.score = 0;
    }

    /// Record a soft drop by one cell.
    pub fn record_soft_drop(&mut self) {
        self.add_score(1);
    }

    /// Draw the scoreboard.
    pub fn draw(&self, engine: &mut Engine) {
        const RIGHT_X: usize = Board::SCOREBOARD_X + Scoreboard::WIDTH - 1;

        for x in Board::SCOREBOARD_X..RIGHT_X {
            engine.draw_tile(tileset::CLEAR_TILE, x, Board::SCOREBOARD_Y);
        }

        engine.draw_number(self.score, RIGHT_X, Board::SCOREBOARD_Y);
    }

    /// Add score to the scoreboard.
    fn add_score(&mut self, score: usize) {
        #[allow(clippy::cast_possible_truncation)]
        const MAX_SCORE: usize = usize::pow(10, Scoreboard::WIDTH as u32) - 1;

        self.score = MAX_SCORE.min(self.score + score);
    }
}
