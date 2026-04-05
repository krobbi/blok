use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

use crate::{canvas::Canvas, draw::DrawContext, errors::BlokError, tiles::Tileset, world::World};

/// Runs Blok. This function returns a [`BlokError`] if an error occurred.
pub fn run() -> Result<(), BlokError> {
    let tileset = Tileset::new(tileset_png_data())?;
    let world = World::new();
    let mut app = App::new(world, tileset);
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app)?;
    app.error.map_or(Ok(()), Err)
}

/// A Blok application.
struct App {
    /// The [`World`].
    world: World,

    /// The [`Tileset`].
    tileset: Tileset,

    /// The [`Canvas`], if any.
    canvas: Option<Canvas>,

    /// The first caught [`BlokError`], if any.
    error: Option<BlokError>,
}

impl App {
    /// Creates a new `App` from a [`World`] and a [`Tileset`].
    const fn new(world: World, tileset: Tileset) -> Self {
        Self {
            world,
            tileset,
            canvas: None,
            error: None,
        }
    }

    /// Runs when the `App` is resumed from an [`ActiveEventLoop`]. This
    /// function returns a [`BlokError`] if the `App` could not be resumed.
    fn on_resumed(&mut self, event_loop: &ActiveEventLoop) -> Result<(), BlokError> {
        let canvas = Canvas::new(event_loop)?;
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
                let canvas = self.canvas.as_mut().ok_or("canvas is uninitialized")?;
                let mut draw = DrawContext::new(&self.tileset, canvas.buffer_mut());
                self.world.draw(&mut draw);
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

/// Returns a slice of PNG image data for a [`Tileset`].
const fn tileset_png_data() -> &'static [u8] {
    static PNG_DATA: &[u8] = include_bytes!("../res/tileset.png");

    PNG_DATA
}
