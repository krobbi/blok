use minifb::{Scale, Window, WindowOptions};

/// A game engine.
pub struct Engine {
    /// The window.
    window: Window,
}

impl Engine {
    /// Create a new engine.
    pub fn new() -> Self {
        let window = Window::new(
            "Blok",
            320,
            180,
            WindowOptions {
                scale: Scale::X4,
                ..Default::default()
            },
        )
        .unwrap_or_else(|e| panic!("{e}"));

        Engine { window }
    }

    /// Get whether the window is open.
    pub fn window_open(&self) -> bool {
        self.window.is_open()
    }

    /// Update the engine.
    pub fn update(&mut self) {
        self.window.update();
    }
}
