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
    for y in 0..3 {
        for x in 0..8 {
            let tile = y * 8 + x;
            let (x, y) = (x * 2 + 1, y * 2 + 1);
            engine.draw_tile(tile, x, y);
        }
    }
}
