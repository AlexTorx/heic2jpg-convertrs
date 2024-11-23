use std::path::PathBuf;

use heic2jpeg::utils::generate_jpeg_filename_from_heif;


#[test]
fn test_generate_jpeg_filename_from_heif() -> () {
    // Test function generate_jpeg_filename_from_heif
    
    let heif_file = PathBuf::from("/some/input/file.heif");
    let output_folder = PathBuf::from("/some/output/folder");

    let jpeg_filepath = generate_jpeg_filename_from_heif(&heif_file, &output_folder);

    assert_eq!(jpeg_filepath, PathBuf::from("/some/output/folder/file.jpeg"));
}

#[test]
fn test_generate_jpeg_filename_from_heif_with_filename() -> () {
    // Test function generate_jpeg_filename_from_heif
    
    let heif_file = PathBuf::from("/some/input/file.heif");
    let output_folder = PathBuf::from("/some/output/folder.extension");

    let jpeg_filepath = generate_jpeg_filename_from_heif(&heif_file, &output_folder);

    assert_eq!(jpeg_filepath, PathBuf::from("/some/output/folder.extension/file.jpeg"));
}
