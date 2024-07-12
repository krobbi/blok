use image::{ImageFormat, Rgb, RgbImage};

/// The tile for an empty cell.
pub const CELL_TILE: usize = 0;

/// The tile for a cyan block.
pub const CYAN_BLOCK_TILE: usize = 1;

/// The tile for a blue block.
pub const BLUE_BLOCK_TILE: usize = 2;

/// The tile for an orange block.
pub const ORANGE_BLOCK_TILE: usize = 3;

/// The tile for a yellow block.
pub const YELLOW_BLOCK_TILE: usize = 4;

/// The tile for a green block.
pub const GREEN_BLOCK_TILE: usize = 5;

/// The tile for a purple block.
pub const PURPLE_BLOCK_TILE: usize = 6;

/// The tile for a red block.
pub const RED_BLOCK_TILE: usize = 7;

/// The tile for the background clear color.
pub const CLEAR_TILE: usize = 8;

/// The tile for an empty cell with a cyan ghost.
pub const CYAN_GHOST_TILE: usize = 9;

/// The tile for an empty cell with a blue ghost.
pub const BLUE_GHOST_TILE: usize = 10;

/// The tile for an empty cell with an orange ghost.
pub const ORANGE_GHOST_TILE: usize = 11;

/// The tile for an empty cell with a yellow ghost.
pub const YELLOW_GHOST_TILE: usize = 12;

/// The tile for an empty cell with a green ghost.
pub const GREEN_GHOST_TILE: usize = 13;

/// The tile for an empty cell with a purple ghost.
pub const PURPLE_GHOST_TILE: usize = 14;

/// The tile for an empty cell with a red ghost.
pub const RED_GHOST_TILE: usize = 15;

/// The tile for a border's top-left corner.
pub(super) const TOP_LEFT_BORDER_TILE: usize = 16;

/// The tile for a border's top edge.
pub(super) const TOP_BORDER_TILE: usize = 17;

/// The tile for a border's top-right corner.
pub(super) const TOP_RIGHT_BORDER_TILE: usize = 18;

/// The tile for a border's left edge.
pub(super) const LEFT_BORDER_TILE: usize = 19;

/// The tile for a border's right edge.
pub(super) const RIGHT_BORDER_TILE: usize = 20;

/// The tile for a border's bottom-left corner.
pub(super) const BOTTOM_LEFT_BORDER_TILE: usize = 21;

/// The tile for a border's bottom edge.
pub(super) const BOTTOM_BORDER_TILE: usize = 22;

/// The tile for a border's bottom-right corner.
pub(super) const BOTTOM_RIGHT_BORDER_TILE: usize = 23;

/// The width or height of a tile in pixels.
pub(super) const TILE_SIZE: u32 = 8;

/// The tileset image in PNG format.
static PNG_IMAGE: &[u8] = include_bytes!("../../res/tileset.png");

/// Create a new tileset buffer.
pub(super) fn new() -> Vec<u32> {
    let image = image();
    let width = image.width() / TILE_SIZE;
    let height = image.height() / TILE_SIZE;
    let capacity = usize::try_from(width * height * TILE_SIZE * TILE_SIZE).unwrap();
    let mut tileset = Vec::with_capacity(capacity);

    for y in 0..height {
        for x in 0..width {
            load_tile(&image, x, y, &mut tileset);
        }
    }

    tileset
}

/// Get the tileset image.
fn image() -> RgbImage {
    image::load_from_memory_with_format(PNG_IMAGE, ImageFormat::Png)
        .unwrap_or_else(|e| panic!("{e}"))
        .into_rgb8()
}

/// Load a tile from an image into a tileset buffer.
fn load_tile(image: &RgbImage, x: u32, y: u32, tileset: &mut Vec<u32>) {
    let (x, y) = (x * TILE_SIZE, y * TILE_SIZE);

    for y in y..y + TILE_SIZE {
        for x in x..x + TILE_SIZE {
            load_pixel(image, x, y, tileset);
        }
    }
}

/// Load a pixel from an image into a tileset buffer.
fn load_pixel(image: &RgbImage, x: u32, y: u32, tileset: &mut Vec<u32>) {
    let pixel = *image.get_pixel(x, y);
    let pixel = encode_pixel(pixel);
    tileset.push(pixel);
}

/// Encode an image pixel into a tileset buffer pixel.
fn encode_pixel(pixel: Rgb<u8>) -> u32 {
    let (r, g, b) = pixel.0.into();
    encode_channels(r, g, b)
}

/// Encode a red, green, and blue channel into a tileset buffer pixel.
fn encode_channels(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (u32::from(r), u32::from(g), u32::from(b));
    (r << 16) | (g << 8) | b
}
