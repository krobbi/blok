mod board;
mod piece;
mod shape;

pub use board::Board;
pub use piece::Piece;
pub use shape::Shape;

use crate::engine::Engine;

/// A player's game components.
pub struct Player {
    /// The board.
    board: Board,
}

impl Player {
    /// Create a new player.
    pub fn new() -> Self {
        let board = Board::new();
        Self { board }
    }

    /// Get a reference to the board.
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Get a mutable reference to the board.
    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Draw the player.
    pub fn draw(&self, engine: &mut Engine) {
        self.board.draw(engine);
    }
}
