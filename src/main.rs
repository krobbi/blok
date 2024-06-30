// Don't open console window in release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod engine;

use engine::Engine;

/// Open a window and draw a test scene.
fn main() {
    let mut engine = Engine::new();
    draw_test_scene(&mut engine);

    while engine.window_open() {
        engine.update();
    }
}

/// Draw a test scene.
fn draw_test_scene(engine: &mut Engine) {
    const X: usize = 3;
    const Y: usize = 3;
    const WIDTH: usize = 7;
    const HEIGHT: usize = 3;
    const EMPTY_BLOCK_TILE: usize = 0;
    const CLEAR_TILE: usize = 8;
    const BLOCK_TILE_BASE: usize = 1;
    const GHOST_TILE_BASE: usize = 9;

    for offset in 0..WIDTH {
        let block_tile = BLOCK_TILE_BASE + offset;
        let ghost_tile = GHOST_TILE_BASE + offset;
        let x = X + offset;
        engine.draw_tile(block_tile, x, Y);
        engine.draw_tile(EMPTY_BLOCK_TILE, x, Y + 1);
        engine.draw_tile(ghost_tile, x, Y + 2);
    }

    engine.draw_border(X, Y, WIDTH, HEIGHT);
    engine.draw_tile(CLEAR_TILE, X - 1, Y + HEIGHT + 2);
    engine.draw_border(X + 2, Y + HEIGHT + 3, 0, 0);
    engine.draw_border(X + 5, Y + HEIGHT + 3, 1, 0);
    engine.draw_border(X + 2, Y + HEIGHT + 6, 0, 1);
    engine.draw_border(X + 5, Y + HEIGHT + 6, 1, 1);
    engine.draw_border(1, 1, 38, 20);
}
