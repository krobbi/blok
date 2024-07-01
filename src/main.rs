// Don't open console window in release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod engine;
mod piece;
mod shape;

use engine::{tileset, Engine};
use piece::Piece;
use shape::Shape;

/// Tests a piece.
struct PieceTester {
    /// The piece.
    piece: Piece,

    /// The X position.
    x: usize,

    /// The Y position.
    y: usize,
}

impl PieceTester {
    /// Create a new shape tester from its shape and position.
    fn new(shape: Shape, x: i8, y: i8) -> Self {
        let piece = Piece::new(shape, x, y);
        let (x, y) = Self::cast_position(x, y);
        Self { piece, x, y }
    }

    /// Cast a position from signed to unsigned.
    fn cast_position(x: i8, y: i8) -> (usize, usize) {
        (usize::try_from(x).unwrap(), usize::try_from(y).unwrap())
    }

    /// Get a mutable reference to the piece.
    fn piece_mut(&mut self) -> &mut Piece {
        &mut self.piece
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

        let tile = self.piece.shape().block_tile();

        for (x, y) in self.piece.blocks() {
            let (x, y) = Self::cast_position(x, y);
            engine.draw_tile(tile, x, y);
        }
    }
}

/// Open a window and draw each shape.
fn main() {
    let mut engine = Engine::new();

    let mut piece_testers = vec![
        PieceTester::new(Shape::I, 1, 1),
        PieceTester::new(Shape::O, 7, 1),
        PieceTester::new(Shape::J, 1, 7),
        PieceTester::new(Shape::L, 7, 7),
        PieceTester::new(Shape::S, 13, 7),
        PieceTester::new(Shape::T, 19, 7),
        PieceTester::new(Shape::Z, 25, 7),
    ];

    while engine.window_open() {
        for piece_tester in &mut piece_testers {
            piece_tester.piece_mut().rotate_clockwise();
        }

        for piece_tester in &piece_testers {
            piece_tester.draw(&mut engine);
        }

        engine.update();
    }
}
