use std::fs::{self, File};
use std::io::BufWriter;
use std::path::PathBuf;

use image::DynamicImage;
use libheif_rs::{
    ColorSpace,HeifContext,LibHeif,RgbChroma,
};


pub fn convert_heic_to_jpeg(heic_file: &PathBuf, jpeg_file: &PathBuf) -> () {
    // Create all parent directories if needed
    fs::create_dir_all(jpeg_file.parent().unwrap()).unwrap();

    let lib_heif = LibHeif::new();
    let context = HeifContext::read_from_file(heic_file.to_str().unwrap()).unwrap();
    let handle = context.primary_image_handle().unwrap();

    let image = lib_heif.decode(
        &handle,
        ColorSpace::Rgb(RgbChroma::Rgba),
        None,
    ).unwrap();

    let planes = image.planes();
    let interleaved_planes = planes.interleaved.unwrap();

    let buffer: &[u8] = interleaved_planes.data;
    let rgb_image = image::RgbaImage::from_raw(
        interleaved_planes.width,
        interleaved_planes.height,
        buffer.to_vec(),
    ).unwrap();
    let dyn_image = DynamicImage::ImageRgba8(rgb_image).to_rgb8();

    let mut file = BufWriter::new(File::create(jpeg_file).unwrap());
    dyn_image.write_to(&mut file, image::ImageFormat::Jpeg).unwrap();
}
