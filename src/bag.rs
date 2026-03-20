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
    pub fn debug_draw(&self, ctx: &mut DrawContext<'_, '_>) {
        for (x, shape) in self.shapes.into_iter().enumerate() {
            let tile = if x == usize::from(self.index) {
                shape.block_tile()
            } else {
                shape.ghost_tile()
            };

            ctx.draw_tile(tile, x, 0);
        }
    }
}
