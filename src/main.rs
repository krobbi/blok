// Don't open console window in release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod engine;
mod game;

use engine::Engine;
use game::Game;

/// Run a game.
fn main() {
    let mut engine = Engine::new();
    let mut game = Game::new();

    while engine.window_open() {
        game.update(&engine);
        game.draw(&mut engine);
        engine.update();
    }
}
