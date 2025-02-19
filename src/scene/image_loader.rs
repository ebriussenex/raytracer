use std::path::Path;

use image::{ImageError, ImageReader, RgbImage};

pub fn load_image_to_rgb<T: AsRef<Path>>(filename: T) -> Result<RgbImage, ImageError> {
    let img = ImageReader::open(filename)?.decode()?;
    Ok(img.to_rgb8())
}
