mod board;
mod piece;
mod queue;
mod shape;

pub use board::Board;
pub use piece::Piece;

use queue::Queue;

/// A player's game components.
pub struct Player {
    /// The board.
    board: Board,

    /// The queue.
    queue: Queue,
}

impl Player {
    /// Create a new player.
    pub fn new() -> Self {
        let board = Board::new();
        let queue = Queue::new();
        Self { board, queue }
    }

    /// Get a reference to the board.
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Get a mutable reference to the board.
    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Get a mutable reference to the queue.
    pub fn queue_mut(&mut self) -> &mut Queue {
        &mut self.queue
    }
}
