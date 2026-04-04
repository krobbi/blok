use std::io::{self, Write as _};

use pixels::TextureError;
use thiserror::Error;
use winit::error::{EventLoopError, OsError};

/// An error caught by Blok.
#[derive(Debug, Error)]
#[repr(transparent)]
#[error(transparent)]
pub struct BlokError(Box<Kind>);

impl BlokError {
    /// Prints the `BlokError`, ignoring any [`io::Error`]s caused by printing.
    pub fn print(&self) {
        // Printing has no effect in Windows release builds, so returning early
        // may eliminate some dead code.
        if cfg!(all(not(debug_assertions), target_os = "windows")) {
            return;
        }

        let _ = writeln!(io::stderr(), "{self}");
    }
}

impl<E: Into<Kind>> From<E> for BlokError {
    #[cold]
    fn from(value: E) -> Self {
        Self(Box::new(value.into()))
    }
}

/// A kind of [`BlokError`].
#[derive(Debug, Error)]
#[error("Error: {0}")]
enum Kind {
    /// A static error message.
    Message(&'static str),

    /// An [`EventLoopError`].
    EventLoop(#[from] EventLoopError),

    /// An [`OsError`].
    Os(#[from] OsError),

    /// A [`pixels::Error`].
    Pixels(#[from] pixels::Error),

    /// A [`TextureError`].
    Texture(#[from] TextureError),
}

impl From<&'static str> for Kind {
    fn from(value: &'static str) -> Self {
        Self::Message(value)
    }
}
