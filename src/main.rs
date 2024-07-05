// Don't open console window in release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod engine;
mod piece;
mod shape;

use board::Board;
use engine::{Engine, Key};
use shape::Shape;

/// Open a window and draw each shape.
fn main() {
    const DROP_PERIOD: f64 = 0.2;
    const LOCK_PERIOD: f64 = 0.5;

    let mut engine = Engine::new();
    let mut board = Board::new();
    let mut piece = board.create_piece(Shape::J);
    let mut drop_timer = DROP_PERIOD;
    let mut lock_timer = LOCK_PERIOD;

    while engine.window_open() {
        if engine.key_pressed(Key::Left) {
            piece.move_left(&board);
        } else if engine.key_pressed(Key::Right) {
            piece.move_right(&board);
        } else if engine.key_pressed(Key::Up) {
            piece.rotate_clockwise(&board);
        } else if engine.key_pressed(Key::Z) {
            piece.rotate_counter_clockwise(&board);
        }

        if piece.airborne(&board) {
            drop_timer -= engine.delta();
            lock_timer = LOCK_PERIOD;

            if drop_timer <= 0.0 {
                drop_timer += DROP_PERIOD;
                piece.drop(&board);
            }
        } else {
            lock_timer -= engine.delta();
            drop_timer = DROP_PERIOD;

            if lock_timer <= 0.0 {
                board.lock_piece(piece);
                piece = board.create_piece(Shape::J);
                drop_timer = DROP_PERIOD;
            }
        }

        board.draw(&mut engine);
        Board::draw_piece(piece, &mut engine);
        engine.update();
    }
}
