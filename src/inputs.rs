use minifb::Key;

/// An input.
#[derive(Clone, Copy)]
pub enum Input {
    /// A hard drop.
    HardDrop,
}

impl Input {
    /// Returns the `Input`'s [`Key`].
    pub const fn key(self) -> Key {
        match self {
            Self::HardDrop => Key::Space,
        }
    }
}
