mod tileset;

use minifb::{Scale, Window, WindowOptions};

/// A game engine.
pub struct Engine {
    /// The window.
    window: Window,

    /// The frame buffer.
    buffer: Vec<u32>,

    /// The tileset buffer.
    tileset: Vec<u32>,
}

impl Engine {
    /// The width of an engine's frame buffer in pixels.
    const WIDTH: usize = 320;

    /// The height of an engine's frame buffer in pixels.
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

        let buffer = vec![0x2e_26_4d; Self::WIDTH * Self::HEIGHT];
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

    /// Draw a tile.
    pub fn draw_tile(&mut self, tile: usize, x: usize, y: usize) {
        const TILE_SIZE: usize = tileset::TILE_SIZE as usize;
        const TILES_ACROSS: usize = Engine::WIDTH / TILE_SIZE;
        const TILES_DOWN: usize = Engine::HEIGHT / TILE_SIZE;
        const OFFSET_X: usize = (Engine::WIDTH - TILES_ACROSS * TILE_SIZE) / 2;
        const OFFSET_Y: usize = (Engine::HEIGHT - TILES_DOWN * TILE_SIZE) / 2;

        let mut tileset_index = tile * (TILE_SIZE * TILE_SIZE);
        let mut buffer_index = (y * TILE_SIZE + OFFSET_Y) * Self::WIDTH + x * TILE_SIZE + OFFSET_X;

        for _ in 0..TILE_SIZE {
            self.buffer[buffer_index..buffer_index + TILE_SIZE]
                .copy_from_slice(&self.tileset[tileset_index..tileset_index + TILE_SIZE]);

            tileset_index += TILE_SIZE;
            buffer_index += Self::WIDTH;
        }
    }

    /// Update the engine.
    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, Self::WIDTH, Self::HEIGHT)
            .unwrap();
    }
}
