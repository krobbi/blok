mod bag;

use bag::Bag;

use crate::{engine::Engine, game::player::Board};

use super::shape::Shape;

/// An infinite queue of random, evenly distributed shapes.
pub struct Queue {
    /// The bag feeding the queue.
    bag: Bag,

    /// The shapes in the queue.
    shapes: [Shape; Self::LENGTH],
}

impl Queue {
    /// The length of a queue in shapes.
    const LENGTH: usize = 6;

    /// Create a new queue.
    pub fn new() -> Self {
        let mut bag = Bag::new();
        let mut shapes = [Shape::I; Self::LENGTH];
        shapes.fill_with(|| bag.shape());
        Self { bag, shapes }
    }

    /// Reset the queue.
    pub fn reset(&mut self) {
        self.bag.reset();
        self.shapes.fill_with(|| self.bag.shape());
    }

    /// Get the next shape from the queue.
    pub fn shape(&mut self) -> Shape {
        let shape = self.shapes[0];
        self.shapes.copy_within(1..Self::LENGTH, 0);
        self.shapes[Self::LENGTH - 1] = self.bag.shape();
        shape
    }

    /// Draw the queue.
    pub fn draw(&self, engine: &mut Engine) {
        const STEP: usize = Shape::HEIGHT + 1;
        const WIDTH: usize = 1 + Shape::WIDTH + 1;
        const HEIGHT: usize = 1 + Queue::LENGTH * STEP;

        engine.draw_border(Board::QUEUE_X, Board::QUEUE_Y, WIDTH, HEIGHT);

        for (y, shape) in self
            .shapes
            .iter()
            .enumerate()
            .map(|(y, shape)| (y * STEP + Board::QUEUE_Y + 1, shape))
        {
            const X: usize = Board::QUEUE_X + 1;

            shape.draw(X, y, engine);
        }
    }
}
