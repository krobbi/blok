mod board;
mod hold;
mod piece;
mod queue;
mod shape;

pub use board::Board;
pub use piece::Piece;
pub use shape::Shape;

use hold::Hold;
use queue::Queue;

/// A player's game components.
pub struct Player {
    /// The board.
    board: Board,

    /// The hold.
    hold: Hold,

    /// The queue.
    queue: Queue,
}

impl Player {
    /// Create a new player.
    pub fn new() -> Self {
        let board = Board::new();
        let hold = Hold::new();
        let queue = Queue::new();
        Self { board, hold, queue }
    }

    /// Get a reference to the board.
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Get a mutable reference to the board.
    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Get a reference to the hold.
    pub fn hold(&self) -> &Hold {
        &self.hold
    }

    /// Get a mutable reference to the hold.
    pub fn hold_mut(&mut self) -> &mut Hold {
        &mut self.hold
    }

    /// Get a reference to the queue.
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    /// Get a mutable reference to the queue.
    pub fn queue_mut(&mut self) -> &mut Queue {
        &mut self.queue
    }
}
