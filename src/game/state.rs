mod creating_piece;
mod dropping_piece;
mod game_over;
mod new_game;

pub use creating_piece::CreatingPiece;
pub use dropping_piece::DroppingPiece;
pub use game_over::GameOver;
pub use new_game::NewGame;

use crate::engine::Engine;

use super::player::Player;

/// An optional transition to a new state.
type Transition = Option<Box<dyn State>>;

/// A player's game state.
pub trait State {
    /// Update the state and get the transition.
    fn update(&mut self, player: &mut Player, engine: &Engine) -> Transition;

    /// Draw the state.
    fn draw(&self, _engine: &mut Engine) {}

    /// Wrap the state in a transition.
    fn transition(self) -> Transition
    where
        Self: Sized + 'static,
    {
        Some(Box::new(self))
    }
}
