mod bag;
mod board;
mod piece;
mod shape;

pub use board::Board;
pub use piece::Piece;

use bag::Bag;
use shape::Shape;

/// A player's game components.
pub struct Player {
    /// The board.
    board: Board,

    /// The bag.
    bag: Bag,
}

impl Player {
    /// Create a new player.
    pub fn new() -> Self {
        let board = Board::new();
        let bag = Bag::new();
        Self { board, bag }
    }

    /// Get a reference to the board.
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Get a mutable reference to the board.
    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Get a mutable reference to the bag.
    pub fn bag_mut(&mut self) -> &mut Bag {
        &mut self.bag
    }
}
