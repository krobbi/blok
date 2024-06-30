// Don't open console window in release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod engine;
mod shape;

use engine::{tileset, Engine};
use shape::Shape;

/// Tests a shape.
struct ShapeTester {
    /// The shape.
    shape: Shape,

    /// The X position.
    x: usize,

    /// The Y position.
    y: usize,
}

impl ShapeTester {
    /// Create a new shape tester from its shape and position.
    fn new(shape: Shape, x: usize, y: usize) -> Self {
        Self { shape, x, y }
    }

    /// Draw the shape tester.
    fn draw(&self, engine: &mut Engine) {
        const WIDTH: usize = 4;
        const HEIGHT: usize = 4;

        let x = self.x;
        let y = self.y;
        engine.draw_border(x, y, WIDTH, HEIGHT);

        for x in x..x + WIDTH {
            for y in y..y + HEIGHT {
                engine.draw_tile(tileset::CELL_TILE, x, y);
            }
        }

        engine.draw_tile(self.shape.block_tile(), x, y);
    }
}

/// Open a window and draw each shape.
fn main() {
    let mut engine = Engine::new();

    let shape_testers = vec![
        ShapeTester::new(Shape::I, 1, 1),
        ShapeTester::new(Shape::O, 7, 1),
        ShapeTester::new(Shape::J, 1, 7),
        ShapeTester::new(Shape::L, 7, 7),
        ShapeTester::new(Shape::S, 13, 7),
        ShapeTester::new(Shape::T, 19, 7),
        ShapeTester::new(Shape::Z, 25, 7),
    ];

    while engine.window_open() {
        for shape_tester in &shape_testers {
            shape_tester.draw(&mut engine);
        }

        engine.update();
    }
}
