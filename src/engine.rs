mod tileset;

use minifb::{Scale, Window, WindowOptions};

/// A game engine.
pub struct Engine {
    /// The window.
    window: Window,

    /// The frame buffer.
    buffer: Vec<u32>,

    /// The tileset buffer.
    // TODO: Use tileset and remove annotation.
    #[allow(dead_code)]
    tileset: Vec<u32>,
}

impl Engine {
    /// The width of the frame buffer in pixels.
    const WIDTH: usize = 320;

    /// The height of the frame buffer in pixels.
    const HEIGHT: usize = 180;

    /// Create a new engine.
    pub fn new() -> Self {
        let window = Window::new(
            "Blok",
            Self::WIDTH,
            Self::HEIGHT,
            WindowOptions {
                scale: Scale::X4,
                ..Default::default()
            },
        )
        .unwrap_or_else(|e| panic!("{e}"));

        let buffer = vec![0; Self::WIDTH * Self::HEIGHT];
        let tileset = tileset::new();

        Engine {
            window,
            buffer,
            tileset,
        }
    }

    /// Get whether the window is open.
    pub fn window_open(&self) -> bool {
        self.window.is_open()
    }

    /// Update the engine.
    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, Self::WIDTH, Self::HEIGHT)
            .unwrap();
    }
}
