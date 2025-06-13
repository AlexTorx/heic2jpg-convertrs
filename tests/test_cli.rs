use assert_cmd::prelude::*;
use std::process::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn heif2jpeg_cli_invalid_input_does_not_exist() {
    // Expect a failure as input will not exist 
    Command::cargo_bin("heic2jpeg")
        .unwrap()
        .args(&["/some/input/folder/that/does/not/exist", "/tmp"])
        .assert()
        .failure();
}

#[test]
fn heif2jpeg_cli_help() {
    // Test help command
    Command::cargo_bin("heic2jpeg")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage"));
}

#[test]
fn heif2jpeg_cli_version() {
    // Test version command
    Command::cargo_bin("heic2jpeg")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn heif2jpeg_cli_invalid_workers() {
    // Test with invalid number of workers
    Command::cargo_bin("heic2jpeg")
        .unwrap()
        .args(&["--workers", "0", "/tmp/input.heic", "/tmp/output.jpg"])
        .assert()
        .failure();
}

#[test]
fn heif2jpeg_cli_missing_arguments() {
    // Test with missing required arguments
    Command::cargo_bin("heic2jpeg")
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn heif2jpeg_cli_directory_conversion() {
    // Create temporary directories for testing
    let temp_dir = TempDir::new().unwrap();
    let input_dir = temp_dir.path().join("input");
    let output_dir = temp_dir.path().join("output");
    
    // Create input directory and copy the test HEIC file
    fs::create_dir(&input_dir).unwrap();
    fs::copy("tests/fixtures/test.heic", input_dir.join("test.heic")).unwrap();
    
    // Create output directory
    fs::create_dir(&output_dir).unwrap();
    
    // Test directory conversion
    Command::cargo_bin("heic2jpeg")
        .unwrap()
        .args(&[
            input_dir.to_str().unwrap(),
            output_dir.to_str().unwrap(),
        ])
        .assert()
        .success();
    
    // Verify that the output file exists
    assert!(output_dir.join("test.jpeg").exists());
}
