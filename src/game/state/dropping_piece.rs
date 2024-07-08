use crate::{
    engine::{Engine, Key},
    game::{
        player::{Board, Piece, Player},
        state,
    },
};

use super::{State, Transition};

/// A state where a piece is dropped on a board.
pub struct DroppingPiece {
    /// The piece.
    piece: Piece,

    /// The time until the piece drops in seconds.
    drop_timer: f64,

    /// The time until the piece locks in seconds.
    lock_timer: f64,
}

impl DroppingPiece {
    /// The time between the piece dropping in seconds.
    const DROP_PERIOD: f64 = 0.2;

    /// The time between the piece landing and locking in seconds.
    const LOCK_PERIOD: f64 = 0.5;

    /// Crete a new dropping piece state from its piece.
    pub fn new(piece: Piece) -> Self {
        let drop_timer = Self::DROP_PERIOD;
        let lock_timer = Self::LOCK_PERIOD;

        Self {
            piece,
            drop_timer,
            lock_timer,
        }
    }

    /// Lock the piece on a board and get the transition.
    fn lock_piece(&self, board: &mut Board) -> Transition {
        if board.lock_piece(self.piece) {
            board.clear_lines();
            state::CreatingPiece::new().transition()
        } else {
            state::GameOver.transition()
        }
    }
}

impl State for DroppingPiece {
    fn update(&mut self, player: &mut Player, engine: &Engine) -> Transition {
        if engine.key_pressed(Key::Left) {
            self.piece.move_left(player.board());
        } else if engine.key_pressed(Key::Right) {
            self.piece.move_right(player.board());
        } else if engine.key_pressed(Key::Up) {
            self.piece.rotate_clockwise(player.board());
        } else if engine.key_pressed(Key::Z) {
            self.piece.rotate_counter_clockwise(player.board());
        } else if engine.key_pressed(Key::Space) {
            self.piece.hard_drop(player.board());
            return self.lock_piece(player.board_mut());
        }

        if self.piece.airborne(player.board()) {
            self.lock_timer = Self::LOCK_PERIOD;

            self.drop_timer -= engine.delta()
                * if engine.key_down(Key::Down) {
                    20.0
                } else {
                    1.0
                };

            if self.drop_timer <= 0.0 {
                self.drop_timer += Self::DROP_PERIOD;
                self.piece.drop(player.board());
            }
        } else {
            self.drop_timer = Self::DROP_PERIOD;
            self.lock_timer -= engine.delta();

            if self.lock_timer <= 0.0 {
                return self.lock_piece(player.board_mut());
            }
        }

        None
    }

    fn draw(&self, player: &Player, engine: &mut Engine) {
        player.board().draw_ghost_piece(self.piece, engine);
        Board::draw_piece(self.piece, engine);
    }
}
