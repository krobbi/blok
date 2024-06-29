use image::{ImageFormat, Rgb, RgbImage};

/// The width or height of a tile in pixels.
const TILE_SIZE: u32 = 8;

/// The tileset image in PNG format.
static PNG_IMAGE: &[u8] = include_bytes!("../../res/tileset.png");

/// Create a new tileset buffer.
pub fn new() -> Vec<u32> {
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
