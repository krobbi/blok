use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::Window};

use crate::errors::BlokError;

/// A canvas with a frame buffer.
#[repr(transparent)]
pub struct Canvas(Pixels<'static>);

impl Canvas {
    /// A `Canvas`' width in graphical pixels.
    pub const WIDTH: usize = 320;

    /// A `Canvas`' height in graphical pixels.
    pub const HEIGHT: usize = 180;

    /// A `Canvas`' channel count.
    pub const CHANNEL_COUNT: usize = 4;

    /// A `Canvas`' clear color as pixels components in RGBA order.
    const CLEAR_COLOR: [u8; Self::CHANNEL_COUNT] = [0x0d, 0x07, 0x09, 0xff];

    /// Creates a new `Canvas` from an [`ActiveEventLoop`]. This function
    /// returns a [`BlokError`] if a `Canvas` could not be created.
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self, BlokError> {
        const SCALE: f64 = 4.0;

        let (width, height) = (
            Self::WIDTH.try_into().expect("width should fit in a `u32`"),
            Self::HEIGHT
                .try_into()
                .expect("height should fit in a `u32`"),
        );

        let size = LogicalSize::new(f64::from(width), f64::from(height));
        let scaled_size = LogicalSize::new(size.width * SCALE, size.height * SCALE);
        let window = event_loop.create_window(
            Window::default_attributes()
                .with_title("Blok")
                .with_inner_size(scaled_size)
                .with_min_inner_size(size),
        )?;

        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(width, height, window);
        let mut pixels = Pixels::new(width, height, surface_texture)?;

        for components in pixels.frame_mut().chunks_exact_mut(Self::CHANNEL_COUNT) {
            components.copy_from_slice(&Self::CLEAR_COLOR);
        }

        pixels.resize_surface(window_size.width, window_size.height)?;
        Ok(Self(pixels))
    }

    /// Resizes the `Canvas` to fit a window size. This function returns a
    /// [`BlokError`] if the `Canvas` could not be resized.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), BlokError> {
        self.0.resize_surface(width, height)?;
        Ok(())
    }

    /// Renders the `Canvas`. This function returns a [`BlokError`] if the
    /// `Canvas` could not be rendered.
    pub fn render(&self) -> Result<(), BlokError> {
        self.0.render()?;
        Ok(())
    }
}
