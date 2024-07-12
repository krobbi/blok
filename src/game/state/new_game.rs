use crate::{
    engine::Engine,
    game::{player::Player, state},
};

use super::{State, Transition};

/// A state where a new game is started.
pub struct NewGame;

impl State for NewGame {
    fn update(&mut self, _player: &mut Player, _engine: &Engine) -> Transition {
        state::ChoosingShape::new().transition()
    }
}
