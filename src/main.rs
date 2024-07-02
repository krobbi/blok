// Don't open console window in release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// TODO: Remove this attribute after components have been integrated.
#![allow(dead_code)]

mod board;
mod engine;
mod piece;
mod shape;

use board::Board;
use engine::Engine;

/// Open a window and draw each shape.
fn main() {
    let mut engine = Engine::new();
    let board = Board::new();

    while engine.window_open() {
        board.draw(&mut engine);
        engine.update();
    }
}
