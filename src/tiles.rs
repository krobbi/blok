use image::{ImageFormat, Rgb, RgbImage};
use strum::{EnumCount, EnumIter, IntoEnumIterator as _};

use crate::errors::BlokError;

/// A graphical tile.
#[derive(Clone, Copy, EnumCount, EnumIter)]
#[repr(u8)]
pub enum Tile {
    /// An empty grid cell.
    Cell,

    /// A cyan block.
    CyanBlock,

    /// A blue block.
    BlueBlock,

    /// An orange block.
    OrangeBlock,

    /// A yellow block.
    YellowBlock,

    /// A green block.
    GreenBlock,

    /// A purple block.
    PurpleBlock,

    /// A red block.
    RedBlock,

    /// A solid clear color.
    Clear,

    /// An empty grid cell with a cyan ghost.
    CyanGhost,

    /// An empty grid cell with a blue ghost.
    BlueGhost,

    /// An empty grid cell with a orange ghost.
    OrangeGhost,

    /// An empty grid cell with a yellow ghost.
    YellowGhost,

    /// An empty grid cell with a green ghost.
    GreenGhost,

    /// An empty grid cell with a purple ghost.
    PurpleGhost,

    /// An empty grid cell with a red ghost.
    RedGhost,
}

impl Tile {
    /// A `Tile`'s width in pixels.
    pub const WIDTH: usize = 8;

    /// A `Tile`'s height in pixels.
    pub const HEIGHT: usize = Self::WIDTH;

    /// A `Tile`'s pixel count.
    const PIXEL_COUNT: usize = Self::WIDTH * Self::HEIGHT;

    /// Returns a `Tile`'s size in pixels.
    fn size() -> (u32, u32) {
        let width = u32::try_from(Self::WIDTH).expect("width should fit in a `u32`");
        let height = u32::try_from(Self::HEIGHT).expect("height should fit in a `u32`");
        (width, height)
    }

    /// Returns the `Tile`'s numeric ID.
    const fn id(self) -> u8 {
        self as u8
    }
}

/// A tileset graphic.
#[repr(transparent)]
pub struct Tileset(Box<[u32; Self::PIXEL_COUNT]>);

impl Tileset {
    /// A `Tileset`'s pixel count.
    const PIXEL_COUNT: usize = Tile::COUNT * Tile::PIXEL_COUNT;

    /// Creates a new `Tileset` from a slice of [`u8`]s containing PNG image
    /// data. This function returns a [`BlokError`] if the image could not be
    /// decoded or if the image is too small for a `Tileset`.
    pub fn new(png_data: &[u8]) -> Result<Self, BlokError> {
        let image = image::load_from_memory_with_format(png_data, ImageFormat::Png)?;
        let image = image.into_rgb8();
        let (tile_width, tile_height) = Tile::size();
        let (tiles_across, tiles_down) = (image.width() / tile_width, image.height() / tile_height);
        let image_tile_count =
            usize::try_from(tiles_across * tiles_down).expect("target should be at least 32-bit");

        if image_tile_count < Tile::COUNT {
            return Err(BlokError::new("Tileset image is too small"));
        }

        let mut buffer = Vec::with_capacity(Self::PIXEL_COUNT);

        for tile in Tile::iter() {
            let tile_index = u32::from(tile.id());
            let (tile_x, tile_y) = (tile_index % tiles_across, tile_index / tiles_across);
            load_tile_graphic(&image, tile_x, tile_y, &mut buffer);
        }

        let buffer = buffer
            .try_into()
            .expect("tileset pixel count should match definition");

        Ok(Self(buffer))
    }

    /// Returns a reference to an array of [`u32`]s containing the `Tileset`'s
    /// pixels for a [`Tile`] in 0RGB format.
    pub fn tile_pixels(&self, tile: Tile) -> &[u32; Tile::PIXEL_COUNT] {
        let pixel_index = usize::from(tile.id()) * Tile::PIXEL_COUNT;
        self.0[pixel_index..pixel_index + Tile::PIXEL_COUNT]
            .try_into()
            .expect("range length should match tile pixel count")
    }
}

/// Loads a [`Tile`]'s graphic from an [`RgbImage`] into a buffer of [`u32`]
/// pixels in 0RGB format.
fn load_tile_graphic(image: &RgbImage, tile_x: u32, tile_y: u32, buffer: &mut Vec<u32>) {
    let (tile_width, tile_height) = Tile::size();
    let (pixel_x, pixel_y) = (tile_x * tile_width, tile_y * tile_height);

    for pixel_y in pixel_y..pixel_y + tile_height {
        for pixel_x in pixel_x..pixel_x + tile_width {
            let pixel = *image.get_pixel(pixel_x, pixel_y);
            let pixel = encode_pixel(pixel);
            buffer.push(pixel);
        }
    }
}

/// Encodes an [`Rgb`] pixel as a [`u32`] pixel in 0RGB format.
fn encode_pixel(pixel: Rgb<u8>) -> u32 {
    let [r, g, b] = pixel.0;
    let (r, g, b) = (u32::from(r), u32::from(g), u32::from(b));
    (r << 16) | (g << 8) | b
}
