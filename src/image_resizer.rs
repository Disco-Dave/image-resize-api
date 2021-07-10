use std::path::Path;

use image::{
    imageops::FilterType, io::Reader as ImageReader, ImageFormat, ImageOutputFormat, ImageResult,
};

fn convert_format(input_format: Option<ImageFormat>) -> ImageOutputFormat {
    match input_format {
        Some(ImageFormat::Png) => ImageOutputFormat::Png,
        Some(ImageFormat::Gif) => ImageOutputFormat::Gif,
        _ => ImageOutputFormat::Jpeg(u8::MAX),
    }
}

pub fn resize(
    path: impl AsRef<Path>,
    max_width: Option<u32>,
    max_height: Option<u32>,
) -> ImageResult<Vec<u8>> {
    let reader = ImageReader::open(path)?;
    let format = reader.format();
    let image = reader.decode()?;

    let resized_image = match (max_width, max_height) {
        (None, None) => image,
        (None, Some(max_height)) => image.resize(u32::MAX, max_height, FilterType::Nearest),
        (Some(max_width), None) => image.resize(max_width, u32::MAX, FilterType::Nearest),
        (Some(max_width), Some(max_height)) => {
            image.resize(max_width, max_height, FilterType::Nearest)
        }
    };

    let mut output = Vec::new();
    resized_image.write_to(&mut output, convert_format(format))?;

    Ok(output)
}
