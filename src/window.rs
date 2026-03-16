use minifb::{Scale, WindowOptions};

use crate::errors::BlokError;

/// A window.
#[repr(transparent)]
pub struct Window(minifb::Window);

impl Window {
    /// Creates a new `Window`. This function returns a [`BlokError`] if a
    /// `Window` could not be created.
    pub fn new() -> Result<Self, BlokError> {
        let window = minifb::Window::new(
            "Blok",
            320,
            180,
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

    /// Updates the `Window`.
    pub fn update(&mut self) {
        self.0.update();
    }
}
