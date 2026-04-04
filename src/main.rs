// Don't open console window in Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod canvas;
mod errors;

use std::process::ExitCode;

/// Runs Blok and returns an [`ExitCode`].
fn main() -> ExitCode {
    match app::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            error.print();
            ExitCode::FAILURE
        }
    }
}
