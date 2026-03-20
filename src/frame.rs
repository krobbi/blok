/// A frame buffer.
#[repr(transparent)]
pub struct Frame(Box<[u32; Self::PIXEL_COUNT]>);

impl Frame {
    /// A `Frame`'s width in pixels.
    pub const WIDTH: usize = 320;

    /// A `Frame`'s height in pixels.
    pub const HEIGHT: usize = 180;

    /// A `Frame`'s pixel count.
    pub const PIXEL_COUNT: usize = Self::WIDTH * Self::HEIGHT;

    /// A `Frame`'s clear color.
    const CLEAR_COLOR: u32 = 0x0d_07_09;

    /// Creates a new `Frame`.
    pub fn new() -> Self {
        let buffer = vec![Self::CLEAR_COLOR; Self::PIXEL_COUNT]
            .try_into()
            .expect("frame buffer pixel count should match definition");

        Self(buffer)
    }

    /// Returns a reference to an array of [`u32`]s containing the `Frame`'s
    /// pixels in 0RGB format.
    pub const fn as_array(&self) -> &[u32; Self::PIXEL_COUNT] {
        &self.0
    }

    /// Returns a reference to a mutable array of [`u32`]s containing the
    /// `Frame`'s pixels in 0RGB format.
    pub const fn as_mut_array(&mut self) -> &mut [u32; Self::PIXEL_COUNT] {
        &mut self.0
    }
}
