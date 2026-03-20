// TODO: Consider using `winit` and `softbuffer` instead of `minifb`.
// Documentation for the latest versions of `winit` is currently broken:
// https://github.com/rust-windowing/winit/issues/4511

use minifb::{KeyRepeat, Scale, WindowOptions};

use crate::{errors::BlokError, frame::Frame, inputs::Input};

/// A window.
#[repr(transparent)]
pub struct Window(minifb::Window);

impl Window {
    /// Creates a new `Window`. This function returns a [`BlokError`] if a
    /// `Window` could not be created.
    pub fn new() -> Result<Self, BlokError> {
        let window = minifb::Window::new(
            "Blok",
            Frame::WIDTH,
            Frame::HEIGHT,
            WindowOptions {
                scale: Scale::X4,
                ..Default::default()
            },
        )?;

        Ok(Self(window))
    }

    /// Returns [`true`] if the `Window` is open.
    pub fn is_open(&self) -> bool {
        self.0.is_open()
    }

    /// Returns [`true`] if an [`Input`] is pressed.
    pub fn is_input_pressed(&self, input: Input) -> bool {
        self.0.is_key_pressed(input.key(), KeyRepeat::No)
    }

    /// Updates the `Window` with a [`Frame`]. This function returns a
    /// [`BlokError`] if the `Window` could not be updated.
    pub fn update_with_frame(&mut self, frame: &Frame) -> Result<(), BlokError> {
        self.0
            .update_with_buffer(frame.as_array(), Frame::WIDTH, Frame::HEIGHT)
            .map_err(BlokError::from)
    }
}
