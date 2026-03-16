// Don't open console window in Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod errors;
mod frame;
mod window;

use std::process::ExitCode;

use crate::{errors::BlokError, frame::Frame, window::Window};

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
    let frame = Frame::new();
    let mut window = Window::new(&frame)?;

    while window.is_open() {
        window.update_with_frame(&frame)?;
    }

    Ok(())
}
