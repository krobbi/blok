mod bag;

use std::array;

use crate::{draw::DrawContext, shapes::Shape};

use self::bag::Bag;

/// A queue of [`Shape`]s.
pub struct Queue {
    /// The [`Bag`] feeding the `Queue`.
    bag: Bag,

    /// The [`Shape`]s.
    shapes: [Shape; Self::LENGTH],
}

impl Queue {
    /// A `Queue`'s length in [`Shape`]s.
    const LENGTH: usize = 6;

    /// Creates a new `Queue`.
    pub fn new() -> Self {
        let mut bag = Bag::new();
        let shapes = array::from_fn(|_| bag.next_shape());
        Self { bag, shapes }
    }

    /// Returns the next [`Shape`] from the `Queue`.
    pub fn next_shape(&mut self) -> Shape {
        let shape = *self.shapes.first().expect("queue should not be empty");
        self.shapes.copy_within(1..Self::LENGTH, 0);
        *self.shapes.last_mut().expect("queue should not be empty") = self.bag.next_shape();
        shape
    }

    /// Draws the `Queue` with a [`DrawContext`] for debugging purposes.
    pub fn debug_draw(&self, x: usize, y: usize, ctx: &mut DrawContext<'_, '_>) {
        ctx.draw_border(x, y, 1, Self::LENGTH);

        for (index, shape) in self.shapes.into_iter().enumerate() {
            ctx.draw_tile(shape.block_tile(), x, y + index);
        }

        self.bag.debug_draw(x + 3, y, ctx);
    }
}
