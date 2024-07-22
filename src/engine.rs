pub mod tileset;

pub use minifb::Key;

use std::time::Instant;

use minifb::{KeyRepeat, Scale, Window, WindowOptions};

/// A game engine.
pub struct Engine {
    /// The window.
    window: Window,

    /// The frame buffer.
    buffer: Vec<u32>,

    /// The tileset buffer.
    tileset: Vec<u32>,

    /// The instant of the previous update.
    update: Instant,

    /// The time since the previous update in seconds.
    delta: f64,
}

impl Engine {
    /// The width of an engine's frame buffer in tiles.
    pub const TILES_ACROSS: usize = Self::WIDTH / Self::TILE_SIZE;

    /// The height of an engine's frame buffer in tiles.
    pub const TILES_DOWN: usize = Self::HEIGHT / Self::TILE_SIZE;

    /// The width of an engine's frame buffer in pixels.
    const WIDTH: usize = 320;

    /// The height of an engine's frame buffer in pixels.
    const HEIGHT: usize = 180;

    /// The width or height of a tile in pixels.
    const TILE_SIZE: usize = tileset::TILE_SIZE as usize;

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

        let buffer = vec![0x0d_07_09; Self::WIDTH * Self::HEIGHT];
        let tileset = tileset::new();
        let update = Instant::now();
        let delta = 0.0;

        Engine {
            window,
            buffer,
            tileset,
            update,
            delta,
        }
    }

    /// Get whether the window is open.
    pub fn window_open(&self) -> bool {
        self.window.is_open()
    }

    /// Get the time since the previous update in seconds.
    pub fn delta(&self) -> f64 {
        self.delta
    }

    /// Get whether a key is down.
    pub fn key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    /// Get whether a key is pressed.
    pub fn key_pressed(&self, key: Key) -> bool {
        self.window.is_key_pressed(key, KeyRepeat::No)
    }

    /// Draw a border around a rectangle.
    pub fn draw_border(&mut self, x: usize, y: usize, width: usize, height: usize) {
        let left_x = x - 1;
        let right_x = x + width;
        let top_y = y - 1;
        let bottom_y = y + height;

        self.draw_tile(tileset::TOP_LEFT_BORDER_TILE, left_x, top_y);
        self.draw_tile(tileset::TOP_RIGHT_BORDER_TILE, right_x, top_y);
        self.draw_tile(tileset::BOTTOM_LEFT_BORDER_TILE, left_x, bottom_y);
        self.draw_tile(tileset::BOTTOM_RIGHT_BORDER_TILE, right_x, bottom_y);

        for x in x..right_x {
            self.draw_tile(tileset::TOP_BORDER_TILE, x, top_y);
            self.draw_tile(tileset::BOTTOM_BORDER_TILE, x, bottom_y);
        }

        for y in y..bottom_y {
            self.draw_tile(tileset::LEFT_BORDER_TILE, left_x, y);
            self.draw_tile(tileset::RIGHT_BORDER_TILE, right_x, y);
        }
    }

    /// Draw a number.
    pub fn draw_number(&mut self, mut number: usize, mut x: usize, y: usize) {
        loop {
            const BASE: usize = 10;

            let tile = number % BASE + tileset::DIGIT_TILE_OFFSET;
            self.draw_tile(tile, x, y);
            number /= BASE;

            if number == 0 {
                break;
            }

            x -= 1;
        }
    }

    /// Draw a tile.
    pub fn draw_tile(&mut self, tile: usize, x: usize, y: usize) {
        const OFFSET_X: usize = (Engine::WIDTH - Engine::TILES_ACROSS * Engine::TILE_SIZE) / 2;
        const OFFSET_Y: usize = (Engine::HEIGHT - Engine::TILES_DOWN * Engine::TILE_SIZE) / 2;

        let mut tileset_index = tile * (Self::TILE_SIZE * Self::TILE_SIZE);

        let mut buffer_index =
            (y * Self::TILE_SIZE + OFFSET_Y) * Self::WIDTH + x * Self::TILE_SIZE + OFFSET_X;

        for _ in 0..Self::TILE_SIZE {
            self.buffer[buffer_index..buffer_index + Self::TILE_SIZE]
                .copy_from_slice(&self.tileset[tileset_index..tileset_index + Self::TILE_SIZE]);

            tileset_index += Self::TILE_SIZE;
            buffer_index += Self::WIDTH;
        }
    }

    /// Update the engine.
    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, Self::WIDTH, Self::HEIGHT)
            .unwrap();

        let now = Instant::now();
        self.delta = now.duration_since(self.update).as_secs_f64();
        self.update = now;
    }
}
