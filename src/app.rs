use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

use crate::{canvas::Canvas, errors::BlokError};

/// Runs Blok. This function returns a [`BlokError`] if an error occurred.
pub fn run() -> Result<(), BlokError> {
    let mut app = App::new();
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app)?;
    app.error.map_or(Ok(()), Err)
}

/// A Blok application.
#[derive(Default)]
struct App {
    /// The [`Canvas`], if any.
    canvas: Option<Canvas>,

    /// The first caught [`BlokError`], if any.
    error: Option<BlokError>,
}

impl App {
    /// Creates a new `App`.
    fn new() -> Self {
        Self::default()
    }

    /// Runs when the `App` is resumed from an [`ActiveEventLoop`]. This
    /// function returns a [`BlokError`] if the `App` could not be resumed.
    fn on_resumed(&mut self, event_loop: &ActiveEventLoop) -> Result<(), BlokError> {
        let mut canvas = Canvas::new(event_loop)?;
        draw_test_image(&mut canvas);
        self.canvas = Some(canvas);
        Ok(())
    }

    /// Runs when the `App` receives a [`WindowEvent`] from an
    /// [`ActiveEventLoop`]. This function returns a [`BlokError`] if the
    /// [`WindowEvent`] could not be processed.
    fn on_window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        event: &WindowEvent,
    ) -> Result<(), BlokError> {
        match event {
            WindowEvent::Resized(size) => {
                let canvas = self.canvas.as_mut().ok_or("canvas is uninitialized")?;
                canvas.resize(size.width, size.height)?;
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let canvas = self.canvas.as_ref().ok_or("canvas is uninitialized")?;
                canvas.render()?;
            }
            _ => (),
        }

        Ok(())
    }

    /// Logs a [`BlokError`] and exits an [`ActiveEventLoop`].
    #[cold]
    fn log_error(&mut self, event_loop: &ActiveEventLoop, error: BlokError) {
        self.error.get_or_insert(error);
        event_loop.exit();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(error) = self.on_resumed(event_loop) {
            self.log_error(event_loop, error);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Err(error) = self.on_window_event(event_loop, &event) {
            self.log_error(event_loop, error);
        }
    }
}

/// Draws a temporary test image on a [`Canvas`].
fn draw_test_image(canvas: &mut Canvas) {
    let width = usize::try_from(Canvas::WIDTH).expect("target should be at least 32-bit");

    for (index, components) in canvas.frame_mut().chunks_exact_mut(4).enumerate() {
        let (x, y) = (index % width, index / width);
        let value = u8::try_from((x ^ y) & 0xff).expect("value should be wrapped to `u8`");
        components[0] = value;
        components[1] = if value & 1 != 0 { 0x40 } else { 0x00 };
        components[2] = 0x80;
        components[3] = 0xff;
    }
}
