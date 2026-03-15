// Don't open console window in Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Runs Blok.
fn main() {
    println!("Hello again, Blok!");
}
