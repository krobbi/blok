use crate::{
    engine::Engine,
    game::{player::Player, state},
};

use super::{State, Transition};

/// A state where a new piece is attempted to be created on a board.
pub struct CreatingPiece;

impl State for CreatingPiece {
    fn update(&mut self, player: &mut Player, _engine: &Engine) -> Transition {
        let shape = player.bag_mut().shape();

        match player.board().create_piece(shape) {
            None => state::GameOver.transition(),
            Some(piece) => state::DroppingPiece::new(piece).transition(),
        }
    }
}
