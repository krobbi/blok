// Don't open console window in Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod errors;
mod window;

use std::process::ExitCode;

use crate::{errors::BlokError, window::Window};

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
    let mut window = Window::new()?;

    while window.is_open() {
        window.update();
    }

    Ok(())
}
