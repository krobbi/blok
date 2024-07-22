mod board;
mod hold;
mod piece;
mod queue;
mod scoreboard;
mod shape;

pub use board::Board;
pub use piece::Piece;
pub use shape::Shape;

use hold::Hold;
use queue::Queue;
use scoreboard::Scoreboard;

/// A player's game components.
pub struct Player {
    /// The board.
    board: Board,

    /// The hold.
    hold: Hold,

    /// The queue.
    queue: Queue,

    /// The scoreboard.
    scoreboard: Scoreboard,
}

impl Player {
    /// Create a new player.
    pub fn new() -> Self {
        let board = Board::new();
        let hold = Hold::new();
        let queue = Queue::new();
        let scoreboard = Scoreboard::new();

        Self {
            board,
            hold,
            queue,
            scoreboard,
        }
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

    /// Get a reference to the scoreboard.
    pub fn scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }

    /// Get a mutable reference to the scoreboard.
    pub fn scoreboard_mut(&mut self) -> &mut Scoreboard {
        &mut self.scoreboard
    }
}
