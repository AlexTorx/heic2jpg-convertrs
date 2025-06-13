# heic2jpeg-convertrs

This utility is a CLI (Command-Line Interface) high-performance tool to ease picture
conversion from Apple's HEIC format to regular JPEG-encoded picture.

## Download


## Usage

The tool can be used once installed using the ``heic2jpeg`` command.

heic2jpeg --input`

## Building

If instead you wish to compile this yourself it is highly recommended to use [wasm-pack](https://github.com/rustwasm/wasm-pack).

Compilation is done as follows:

```
wasm-pack build --target web --release
```

The resulting files are found in the `pkg` directory. At minimum, you will need at least `rust-template_bg.wasm` and `rust-template.js`.
