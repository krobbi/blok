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
    draw_test_numbers(&mut engine);

    while engine.window_open() {
        game.update(&engine);
        game.draw(&mut engine);
        engine.update();
    }
}

/// Draw test numbers.
fn draw_test_numbers(engine: &mut Engine) {
    engine.draw_number(0, 5, 0);
    engine.draw_number(1, 5, 1);
    engine.draw_number(23, 5, 2);
    engine.draw_number(40, 5, 3);
    engine.draw_number(567, 5, 4);
    engine.draw_number(8900, 5, 5);
    engine.draw_number(10110, 5, 6);
}
