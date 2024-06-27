// Don't open console window in release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod engine;

use engine::Engine;

/// Open a window.
fn main() {
    let mut engine = Engine::new();

    while engine.window_open() {
        engine.update();
    }
}
