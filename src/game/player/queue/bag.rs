use rand::seq::SliceRandom;

use crate::game::player::Shape;

/// An infinite bag generating a random, even distribution of shapes.
pub struct Bag {
    /// The index into the set of shapes.
    index: usize,

    /// The shuffled set of each shape.
    shapes: [Shape; Shape::COUNT],
}

impl Bag {
    /// Create a new bag.
    pub fn new() -> Self {
        let index = 0;

        let mut shapes = [
            Shape::I,
            Shape::J,
            Shape::L,
            Shape::O,
            Shape::S,
            Shape::T,
            Shape::Z,
        ];

        Self::shuffle(&mut shapes);
        Self { index, shapes }
    }

    /// Reset the bag.
    pub fn reset(&mut self) {
        self.index = 0;
        Self::shuffle(&mut self.shapes);
    }

    /// Get the next shape from the bag.
    pub fn shape(&mut self) -> Shape {
        let shape = self.shapes[self.index];

        if self.index == Shape::COUNT - 1 {
            self.reset();
        } else {
            self.index += 1;
        }

        shape
    }

    /// Shuffle a set of shapes.
    fn shuffle(shapes: &mut [Shape; Shape::COUNT]) {
        shapes.shuffle(&mut rand::thread_rng());
    }
}
