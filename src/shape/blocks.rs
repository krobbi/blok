use super::Shape;

/// A set of block positions forming a shape.
pub type Blocks = [(i8, i8); 4];

/// Create new blocks from a shape.
pub fn new(shape: Shape) -> Blocks {
    BLOCKS[shape as usize]
}

/// The table of blocks, ordered by shape.
#[rustfmt::skip]
static BLOCKS: &[Blocks; Shape::COUNT] = &[
    // Shape::I
    [(0, 1), (1, 1), (2, 1), (3, 1)],

    // Shape::J
    [(0, 0), (0, 1), (1, 1), (2, 1)],

    // Shape::L
    [(2, 0), (0, 1), (1, 1), (2, 1)],

    // Shape::O
    [(1, 0), (2, 0), (1, 1), (2, 1)],

    // Shape::S
    [(1, 0), (2, 0), (0, 1), (1, 1)],

    // Shape::T
    [(1, 0), (0, 1), (1, 1), (2, 1)],

    // Shape::Z
    [(0, 0), (1, 0), (1, 1), (2, 1)],
];
