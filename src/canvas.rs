use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::Window};

use crate::errors::BlokError;

/// A canvas with a frame buffer.
#[repr(transparent)]
pub struct Canvas(Pixels<'static>);

impl Canvas {
    /// The width of a `Canvas` in graphical pixels.
    pub const WIDTH: u32 = 320;

    /// The height of a `Canvas` in graphical pixels.
    pub const HEIGHT: u32 = 180;

    /// Creates a new `Canvas` from an [`ActiveEventLoop`]. This function
    /// returns a [`BlokError`] if a `Canvas` could not be created.
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self, BlokError> {
        const SCALE: f64 = 4.0;

        let size = LogicalSize::new(f64::from(Self::WIDTH), f64::from(Self::HEIGHT));
        let scaled_size = LogicalSize::new(size.width * SCALE, size.height * SCALE);
        let window = event_loop.create_window(
            Window::default_attributes()
                .with_title("Blok")
                .with_inner_size(scaled_size)
                .with_min_inner_size(size),
        )?;

        let surface_texture = SurfaceTexture::new(Self::WIDTH, Self::HEIGHT, window);
        let pixels = Pixels::new(Self::WIDTH, Self::HEIGHT, surface_texture)?;
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

    /// Returns the `Canvas`' frame buffer as a mutable slice of pixel
    /// components in RGBA order.
    pub fn frame_mut(&mut self) -> &mut [u8] {
        self.0.frame_mut()
    }
}
