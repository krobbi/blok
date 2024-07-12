mod player;
mod state;

use player::Player;
use state::State;

use crate::engine::Engine;

/// A single-player game.
pub struct Game {
    /// The player.
    player: Player,

    /// The state.
    state: Box<dyn State>,
}

impl Game {
    /// Create a new game.
    pub fn new() -> Self {
        let player = Player::new();
        let state = Box::new(state::NewGame);
        Self { player, state }
    }

    /// Update the game.
    pub fn update(&mut self, engine: &Engine) {
        while let Some(state) = self.state.update(&mut self.player, engine) {
            self.state = state;
        }
    }

    /// Draw the game.
    pub fn draw(&self, engine: &mut Engine) {
        self.player.board().draw(engine);
        self.player.queue().draw(engine);
        self.state.draw(&self.player, engine);
    }
}
