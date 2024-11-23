use std::path::PathBuf;


pub fn generate_jpeg_filename_from_heif(
    heif_file: PathBuf,
    output_folder: PathBuf,
) -> PathBuf {
    // Creates the target output file from source HEIF filepath
    //
    // # Examples
    //
    // ```
    // use std::path::PathBuf;
    //
    // let input_heif = PathBuf:new("/some/input/heif/file.heif");
    // let output_folder = PathBuf::new("/some/output/folder");
    // ```
    
    let mut path: PathBuf = [
        output_folder.as_os_str(), heif_file.file_stem().unwrap()
    ].iter().collect();
    path.set_extension("jpeg");

    path
}
