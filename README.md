<div align="center">
  <h1 style="font-size: 5em; font-family: 'Arial', sans-serif; color: #2c3e50; margin-bottom: 0.5em;">
    🖼️ HEIC to JPEG Converter
  </h1>
  <p style="color: #7f8c8d; font-size: 1.2em; margin-top: 0;">
    <em>A lightning-fast Rust-powered tool for converting Apple's HEIC images to standard JPEG format</em>
  </p>
</div>


A high-performance command-line tool for converting Apple's HEIC image format to standard JPEG format. Built in Rust, this tool provides fast and reliable conversion of HEIC images while maintaining image quality.

## Features

- Fast and efficient HEIC to JPEG conversion
- Multi-threaded processing for batch conversions
- Maintains original image quality
- Simple command-line interface
- Cross-platform support

## Installation

### From Source

1. Ensure you have Rust and Cargo installed on your system:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```bash
   git clone https://github.com/alextorx/heic2jpg-convertrs.git
   cd heic2jpg-convertrs
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

4. The binary will be available at `target/release/heic2jpeg`

### Dependencies

The following system dependencies are required:

- libheif development files
- JPEG development files

On Ubuntu/Debian:
```bash
sudo apt-get install libheif-dev libjpeg-dev
```

On macOS:
```bash
brew install libheif
```

## Usage

### Basic Usage

Convert a single HEIC file to JPEG:
```bash
heic2jpeg --input path/to/image.heic --output path/to/output.jpg
```

### Command Line Options

```
USAGE:
    heic2jpeg <INPUT> <OUTPUT> [OPTIONS]

ARGS:
    <INPUT>     Input target for conversion (either a file or a folder)
    <OUTPUT>    Target for output of conversion (type must match input option)

OPTIONS:
    -h, --help              Print help information
    -w, --workers <NUM>     Number of workers to spawn for conversion [default: number of CPU cores]
```

The tool supports both single file and directory conversion:
- When INPUT is a file, OUTPUT must be a file path
- When INPUT is a directory, OUTPUT must be a directory path

### Examples

1. Convert a single file:
   ```bash
   heic2jpeg photo.heic photo.jpg
   ```

2. Convert a directory of HEIC files:
   ```bash
   heic2jpeg input_directory/ output_directory/
   ```

3. Convert with specific number of workers:
   ```bash
   heic2jpeg photo.heic photo.jpg --workers 4
   ```

## Development

### Building

Build the project in debug mode:
```bash
cargo build
```

Build in release mode:
```bash
cargo build --release
```

### Running Tests

Run the test suite:
```bash
cargo test
```

Run tests with verbose output:
```bash
cargo test -- --nocapture
```

### Project Structure

```
heic2jpg-convertrs/
├── src/
│   ├── bin/
│   │   └── heic2jpeg.rs    # CLI interface and main program logic
│   ├── convert.rs          # Core HEIC to JPEG conversion functionality
│   ├── lib.rs             # Library entry point
│   ├── threadpool.rs      # Thread pool implementation for parallel processing
│   └── utils.rs           # Utility functions
├── tests/
│   ├── test_cli.rs        # CLI interface tests
│   └── test_utils.rs      # Utility functions tests
├── Cargo.toml             # Project dependencies and metadata
├── Cargo.lock             # Locked dependencies versions
├── LICENSE               # MIT License
└── README.md             # This file
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [libheif-rs](https://github.com/Cykooz/libheif-rs) - Rust bindings for libheif
- [image-rs](https://github.com/image-rs/image) - Rust image processing library
