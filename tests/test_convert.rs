use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use heic2jpeg::convert::convert_heic_to_jpeg;

#[test]
fn test_convert_heic_to_jpeg() {
    // Create a temporary directory for test files
    // Create temporary directories for testing
    let temp_dir = TempDir::new().unwrap();
    let input_dir = temp_dir.path().join("input");
    let output_dir = temp_dir.path().join("output");
    
    // Create input directory and copy the test HEIC file
    fs::create_dir(&input_dir).unwrap();
    fs::copy("tests/fixtures/test.heic", input_dir.join("test.heic")).unwrap();

    // Create files paths for input and output
    let heic_path = input_dir.join("test.heic");
    let jpeg_path = output_dir.join("test.jpg");
    
    // Test conversion
    convert_heic_to_jpeg(&heic_path, &jpeg_path);
    
    // Verify that the output file exists
    assert!(jpeg_path.exists());
}

#[test]
fn test_convert_heic_to_jpeg_invalid_input() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = temp_dir.path();
    
    // Test with non-existent input file
    let heic_path = test_dir.join("nonexistent.heic");
    let jpeg_path = test_dir.join("output.jpg");
    
    // This should panic or return an error
    let result = std::panic::catch_unwind(|| {
        convert_heic_to_jpeg(&heic_path, &jpeg_path);
    });
    
    assert!(result.is_err());
}

#[test]
fn test_convert_heic_to_jpeg_invalid_output_directory() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = temp_dir.path();
    
    // Create test HEIC file
    let heic_path = test_dir.join("test.heic");
    fs::write(&heic_path, b"dummy heic content").unwrap();
    
    // Test with invalid output directory
    let jpeg_path = PathBuf::from("/nonexistent/directory/output.jpg");
    
    // This should panic or return an error
    let result = std::panic::catch_unwind(|| {
        convert_heic_to_jpeg(&heic_path, &jpeg_path);
    });
    
    assert!(result.is_err());
} 