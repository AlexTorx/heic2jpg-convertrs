use std::process::Command;

use assert_cmd::prelude::*;


#[test]
fn heif2jpeg_cli_invalid_input_does_not_exist() -> () {
    // Expect a failure as input will not exist 
    Command::cargo_bin("heic2jpeg")
        .unwrap()
        .args(&["/some/input/folder/that/does/not/exist", "/tmp"])
        .assert()
        .failure();
}
