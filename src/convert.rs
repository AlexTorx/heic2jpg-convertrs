use std::fs;
use std::path::PathBuf;


pub fn convert_heic_to_jpeg(heic_file: &PathBuf, jpeg_file: &PathBuf) -> () {

    // Create all parent directories if needed
    fs::create_dir_all(jpeg_file.parent().unwrap()).unwrap();

    // Copy file from source to target
    fs::copy(heic_file, jpeg_file).unwrap();
}
