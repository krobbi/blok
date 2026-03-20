// Don't open console window in Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod errors;
mod frame;
mod tiles;
mod window;

use std::process::ExitCode;

use crate::{errors::BlokError, frame::Frame, tiles::Tileset, window::Window};

/// Runs Blok and returns an [`ExitCode`].
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            error.print();
            ExitCode::FAILURE
        }
    }
}

/// Runs Blok. This function returns a [`BlokError`] if a fatal error occurred.
fn run() -> Result<(), BlokError> {
    let _tileset = load_tileset()?;
    let mut window = Window::new()?;
    let frame = Frame::new();

    while window.is_open() {
        window.update_with_frame(&frame)?;
    }

    Ok(())
}

/// Creates a new [`Tileset`] from static PNG image data. This function returns
/// a [`BlokError`] if the [`Tileset`] could not be created.
fn load_tileset() -> Result<Tileset, BlokError> {
    static PNG_DATA: &[u8] = include_bytes!("../res/tileset.png");

    Tileset::new(PNG_DATA)
}
