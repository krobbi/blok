// Don't open console window in Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bag;
mod draw;
mod errors;
mod frame;
mod inputs;
mod shapes;
mod tiles;
mod window;

use std::process::ExitCode;

use crate::{
    bag::Bag,
    draw::DrawContext,
    errors::BlokError,
    frame::Frame,
    inputs::Input,
    shapes::Shape,
    tiles::{Tile, Tileset},
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

    let mut bag = Bag::new();
    let mut shape = None;

    while window.is_open() {
        if window.is_input_pressed(Input::HardDrop) {
            shape = Some(bag.next_shape());
        }

        {
            let mut ctx = DrawContext::new(&tileset, &mut frame);
            bag.debug_draw(16, 8, &mut ctx);
            let tile = shape.map_or(Tile::Clear, Shape::block_tile);
            ctx.draw_tile(tile, 16, 11);
            ctx.draw_border(16, 11, 1, 1);
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
