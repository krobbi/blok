// Don't open console window in Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod draw;
mod errors;
mod frame;
mod shapes;
mod tiles;
mod window;

use std::process::ExitCode;

use strum::IntoEnumIterator as _;

use crate::{
    draw::DrawContext, errors::BlokError, frame::Frame, shapes::Shape, tiles::Tileset,
    window::Window,
};

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
    let tileset = load_tileset()?;
    let mut window = Window::new()?;
    let mut frame = Frame::new();

    while window.is_open() {
        {
            let mut ctx = DrawContext::new(&tileset, &mut frame);
            draw_test_scene(&mut ctx);
        }

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

/// Draws a test scene with a [`DrawContext`].
fn draw_test_scene(ctx: &mut DrawContext<'_, '_>) {
    for (x, shape) in Shape::iter().enumerate() {
        ctx.draw_tile(shape.block_tile(), x, 0);
    }
}
