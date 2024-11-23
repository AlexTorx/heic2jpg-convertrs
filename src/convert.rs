use std::fs;
use std::path::PathBuf;

use crate::utils::generate_jpeg_filename_from_heif;


pub fn convert_heic_to_jpeg(heic_file: &PathBuf, output_target: &PathBuf) -> PathBuf {
    let jpeg_file = generate_jpeg_filename_from_heif(heic_file, output_target);

    // Create all parent directories if needed
    fs::create_dir_all(&jpeg_file.parent().unwrap()).unwrap();

    // Copy file from source to target
    fs::copy(heic_file, &jpeg_file).unwrap();

    jpeg_file
}
