// Don't open console window in Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

/// Runs Blok.
fn main() {
    app::run();
}
