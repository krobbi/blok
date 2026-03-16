/// A frame buffer.
#[repr(transparent)]
pub struct Frame(Box<[u32; Self::SIZE]>);

impl Frame {
    /// A `Frame`'s width in pixels.
    const WIDTH: usize = 320;

    /// A `Frame`'s height in pixels.
    const HEIGHT: usize = 180;

    /// A `Frame`'s size in pixels.
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    /// A `Frame`'s clear color.
    const CLEAR_COLOR: u32 = 0x0d_07_09;

    /// Creates a new `Frame`.
    pub fn new() -> Self {
        Self(
            vec![Self::CLEAR_COLOR; Self::SIZE]
                .try_into()
                .expect("frame buffer size should match definition"),
        )
    }

    /// Returns the `Frame`'s width in pixels.
    pub const fn width(&self) -> usize {
        let _ = self;
        Self::WIDTH
    }

    /// Returns the `Frame`'s height in pixels.
    pub const fn height(&self) -> usize {
        let _ = self;
        Self::HEIGHT
    }

    /// Returns a slice of [`u32`]s containing the `Frame`'s pixels in 0RGB
    /// format.
    pub const fn as_slice(&self) -> &[u32] {
        self.0.as_slice()
    }
}
