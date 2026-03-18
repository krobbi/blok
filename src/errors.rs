use std::io::{self, Write as _};

use image::ImageError;
use thiserror::Error;

/// A fatal error caught by Blok.
#[derive(Debug, Error)]
#[repr(transparent)]
#[error(transparent)]
pub struct BlokError(Box<Kind>);

impl<T: Into<Kind>> From<T> for BlokError {
    #[cold]
    fn from(value: T) -> Self {
        Self(value.into().into())
    }
}

impl BlokError {
    /// Creates a new `BlokError` from a static error message.
    #[cold]
    pub fn new(message: &'static str) -> Self {
        Self(Kind::Message(message).into())
    }

    /// Prints the `BlokError`, ignoring any [`io::Error`]s caused by printing.
    pub fn print(&self) {
        let _ = writeln!(io::stderr(), "Error: {self}");
    }
}

/// A [`BlokError`]'s kind.
#[derive(Debug, Error)]
enum Kind {
    /// A static error message.
    #[error("{0}")]
    Message(&'static str),

    /// An [`ImageError`].
    #[error("{0}")]
    Image(#[from] ImageError),

    /// A [`minifb::Error`].
    #[error("{0}")]
    Minifb(#[from] minifb::Error),
}
