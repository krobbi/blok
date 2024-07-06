use crate::{
    engine::Engine,
    game::{
        player::{Player, Shape},
        state,
    },
};

use super::{State, Transition};

/// A state where a new piece is attempted to be created on a board.
pub struct CreatingPiece;

impl State for CreatingPiece {
    fn update(&mut self, player: &mut Player, _engine: &Engine) -> Transition {
        match player.board().create_piece(Shape::J) {
            None => state::GameOver.transition(),
            Some(piece) => state::DroppingPiece::new(piece).transition(),
        }
    }
}
