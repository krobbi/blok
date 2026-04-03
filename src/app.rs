use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use crate::errors::BlokError;

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
    /// The [`Window`], if any.
    window: Option<Window>,

    /// The first caught [`BlokError`], if any.
    error: Option<BlokError>,
}

impl App {
    /// Creates a new `App`.
    fn new() -> Self {
        Self::default()
    }

    /// Logs an error and exits an [`ActiveEventLoop`].
    #[cold]
    fn log_error<E: Into<BlokError>>(&mut self, event_loop: &ActiveEventLoop, error: E) {
        self.error.get_or_insert_with(|| error.into());
        event_loop.exit();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match event_loop.create_window(Window::default_attributes()) {
            Ok(window) => self.window = Some(window),
            Err(error) => self.log_error(event_loop, error),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let Some(window) = &self.window else {
                    self.log_error(event_loop, "window is uninitialized");
                    return;
                };

                window.request_redraw();
            }
            _ => (),
        }
    }
}
