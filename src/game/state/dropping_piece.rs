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
        }

        if self.piece.airborne(player.board()) {
            self.lock_timer = Self::LOCK_PERIOD;
            self.drop_timer -= engine.delta();

            if self.drop_timer <= 0.0 {
                self.drop_timer += Self::DROP_PERIOD;
                self.piece.drop(player.board());
            }
        } else {
            self.drop_timer = Self::DROP_PERIOD;
            self.lock_timer -= engine.delta();

            if self.lock_timer <= 0.0 {
                return if player.board_mut().lock_piece(self.piece) {
                    state::CreatingPiece.transition()
                } else {
                    state::GameOver.transition()
                };
            }
        }

        None
    }

    fn draw(&self, engine: &mut Engine) {
        Board::draw_piece(self.piece, engine);
    }
}
