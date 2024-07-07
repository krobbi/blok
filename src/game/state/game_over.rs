use crate::{
    engine::Engine,
    game::{player::Player, state},
};

use super::{State, Transition};

/// A state where a game is reset after a game over condition.
pub struct GameOver;

impl State for GameOver {
    fn update(&mut self, player: &mut Player, _engine: &Engine) -> Transition {
        player.board_mut().clear();
        player.bag_mut().reset();
        state::NewGame.transition()
    }
}
