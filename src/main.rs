// Don't open console window in release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod engine;
mod piece;
mod shape;

use board::Board;
use engine::{Engine, Key};
use piece::Piece;
use shape::Shape;

/// Open a window and draw each shape.
fn main() {
    let mut engine = Engine::new();
    let board = Board::new();
    let mut piece = Piece::new(Shape::J, 3, -1);

    while engine.window_open() {
        if engine.key_pressed(Key::Up) {
            piece.test_move_up(&board);
        } else if engine.key_pressed(Key::Down) {
            piece.drop(&board);
        } else if engine.key_pressed(Key::Left) {
            piece.move_left(&board);
        } else if engine.key_pressed(Key::Right) {
            piece.move_right(&board);
        } else if engine.key_pressed(Key::X) {
            piece.rotate_clockwise(&board);
        } else if engine.key_pressed(Key::Z) {
            piece.rotate_counter_clockwise(&board);
        }

        board.draw(&mut engine);
        Board::draw_piece(piece, &mut engine);
        engine.update();
    }
}
