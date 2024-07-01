use crate::piece::Facing;

use super::Shape;

/// A set of block positions forming a shape.
pub type Blocks = [(i8, i8); 4];

/// Create new blocks from a shape and a facing.
pub fn new(shape: Shape, facing: Facing) -> Blocks {
    BLOCKS[shape as usize * Facing::COUNT + facing as usize]
}

/// The table of blocks, ordered by shape, and then by facing.
#[rustfmt::skip]
static BLOCKS: &[Blocks; Shape::COUNT * Facing::COUNT] = &[
    // Shape::I
    [(0, 1), (1, 1), (2, 1), (3, 1)], // Facing::Up
    [(2, 0), (2, 1), (2, 2), (2, 3)], // Facing::Right
    [(0, 2), (1, 2), (2, 2), (3, 2)], // Facing::Down
    [(1, 0), (1, 1), (1, 2), (1, 3)], // Facing::Left

    // Shape::J
    [(0, 0), (0, 1), (1, 1), (2, 1)], // Facing::Up
    [(1, 0), (2, 0), (1, 1), (1, 2)], // Facing::Right
    [(0, 1), (1, 1), (2, 1), (2, 2)], // Facing::Down
    [(1, 0), (1, 1), (0, 2), (1, 2)], // Facing::Left

    // Shape::L
    [(2, 0), (0, 1), (1, 1), (2, 1)], // Facing::Up
    [(1, 0), (1, 1), (1, 2), (2, 2)], // Facing::Right
    [(0, 1), (1, 1), (2, 1), (0, 2)], // Facing::Down
    [(0, 0), (1, 0), (1, 1), (1, 2)], // Facing::Left

    // Shape::O
    [(1, 0), (2, 0), (1, 1), (2, 1)], // Facing::Up
    [(1, 0), (2, 0), (1, 1), (2, 1)], // Facing::Right
    [(1, 0), (2, 0), (1, 1), (2, 1)], // Facing::Down
    [(1, 0), (2, 0), (1, 1), (2, 1)], // Facing::Left

    // Shape::S
    [(1, 0), (2, 0), (0, 1), (1, 1)], // Facing::Up
    [(1, 0), (1, 1), (2, 1), (2, 2)], // Facing::Right
    [(1, 1), (2, 1), (0, 2), (1, 2)], // Facing::Down
    [(0, 0), (0, 1), (1, 1), (1, 2)], // Facing::Left

    // Shape::T
    [(1, 0), (0, 1), (1, 1), (2, 1)], // Facing::Up
    [(1, 0), (0, 1), (1, 1), (2, 1)], // Facing::Right
    [(1, 0), (0, 1), (1, 1), (2, 1)], // Facing::Down
    [(1, 0), (0, 1), (1, 1), (2, 1)], // Facing::Left

    // Shape::Z
    [(0, 0), (1, 0), (1, 1), (2, 1)], // Facing::Up
    [(0, 0), (1, 0), (1, 1), (2, 1)], // Facing::Right
    [(0, 0), (1, 0), (1, 1), (2, 1)], // Facing::Down
    [(0, 0), (1, 0), (1, 1), (2, 1)], // Facing::Left
];
