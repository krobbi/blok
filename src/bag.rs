use strum::{EnumCount as _, VariantArray as _};

use crate::{draw::DrawContext, shapes::Shape};

/// A randomly-shuffled bag of [`Shape`]s.
pub struct Bag {
    /// The next [`Shape`]'s index.
    index: u8,

    /// The [`Shape`]s.
    shapes: [Shape; Shape::COUNT],
}

impl Bag {
    /// Creates a new `Bag`.
    pub fn new() -> Self {
        let shapes = Shape::VARIANTS
            .try_into()
            .expect("shape variant array length should match shape count");

        Self { index: 0, shapes }
    }

    /// Draws the `Bag` with a [`DrawContext`] for debugging purposes.
    pub fn debug_draw(&self, x: usize, y: usize, ctx: &mut DrawContext<'_, '_>) {
        ctx.draw_border(x, y, Shape::COUNT, 1);

        for (index, shape) in self.shapes.into_iter().enumerate() {
            let tile = if index == usize::from(self.index) {
                shape.block_tile()
            } else {
                shape.ghost_tile()
            };

            ctx.draw_tile(tile, x + index, y);
        }
    }
}
