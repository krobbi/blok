use image::{ImageFormat, RgbImage};
use strum::EnumCount;

use crate::{canvas::Canvas, errors::BlokError};

/// A graphical tile.
#[derive(Clone, Copy, EnumCount)]
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
}

impl Tile {
    /// A `Tile`'s width in pixels.
    pub const WIDTH: usize = 8;

    /// A `Tile`'s height in pixels.
    pub const HEIGHT: usize = Self::WIDTH;

    /// A `Tile`'s pixel count.
    const PIXEL_COUNT: usize = Self::WIDTH * Self::HEIGHT;

    /// A `Tile`'s pixel component count.
    const COMPONENT_COUNT: usize = Self::PIXEL_COUNT * Canvas::CHANNEL_COUNT;

    /// Returns the `Tile`'s numeric ID.
    const fn id(self) -> u8 {
        self as u8
    }
}

/// A set of graphics for [`Tile`]s.
#[repr(transparent)]
pub struct Tileset(Box<[u8; Self::COMPONENT_COUNT]>);

impl Tileset {
    /// A `Tileset`'s pixel component count.
    const COMPONENT_COUNT: usize = Tile::COUNT * Tile::COMPONENT_COUNT;

    /// Creates a new `Tileset` from a slice of PNG image data. This function
    /// returns a [`BlokError`] if the image could not be decoded or if the
    /// image is too small for a `Tileset`.
    pub fn new(png_data: &[u8]) -> Result<Self, BlokError> {
        let image = image::load_from_memory_with_format(png_data, ImageFormat::Png)?;
        let image = image.into_rgb8();
        let (width, height) = image.dimensions();
        let (width, height) = (
            usize::try_from(width).expect("target should be at least 32-bit"),
            usize::try_from(height).expect("target should be at least 32-bit"),
        );

        let (tiles_across, tiles_down) = (width / Tile::WIDTH, height / Tile::HEIGHT);
        let tile_count = tiles_across * tiles_down;

        if tile_count < Tile::COUNT {
            return Err("tileset image is too small".into());
        }

        let mut buffer = Vec::with_capacity(Self::COMPONENT_COUNT);

        for tile_index in 0..Tile::COUNT {
            let (tile_x, tile_y) = (tile_index % tiles_across, tile_index / tiles_across);
            load_tile_graphic(&image, tile_x, tile_y, &mut buffer);
        }

        let buffer = buffer
            .try_into()
            .expect("buffer length should match tileset pixel component count");

        Ok(Self(buffer))
    }

    /// Returns a [`Tile`]'s graphic as a slice of pixel components in RGBA
    /// order.
    pub fn tile_graphic(&self, tile: Tile) -> &[u8; Tile::COMPONENT_COUNT] {
        let component_index = usize::from(tile.id()) * Tile::COMPONENT_COUNT;
        self.0[component_index..component_index + Tile::COMPONENT_COUNT]
            .try_into()
            .expect("range length should match tile pixel component count")
    }
}

/// Loads a [`Tile`]'s graphic from a tile position in an [`RgbImage`] into a
/// buffer of pixel components in RGBA order.
fn load_tile_graphic(image: &RgbImage, tile_x: usize, tile_y: usize, buffer: &mut Vec<u8>) {
    let (pixel_x, pixel_y) = (tile_x * Tile::WIDTH, tile_y * Tile::HEIGHT);

    for pixel_y in pixel_y..pixel_y + Tile::HEIGHT {
        for pixel_x in pixel_x..pixel_x + Tile::WIDTH {
            let (pixel_x, pixel_y) = (
                pixel_x
                    .try_into()
                    .expect("pixel coordinate should fit in a `u32`"),
                pixel_y
                    .try_into()
                    .expect("pixel coordinate should fit in a `u32`"),
            );

            let components = image.get_pixel(pixel_x, pixel_y).0;
            buffer.extend(components);
            buffer.push(0xff); // Include an opaque alpha component.
        }
    }
}
