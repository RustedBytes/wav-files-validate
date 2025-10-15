# wav-files-validate

A command-line tool to recursively validate the integrity of WAV audio files in a directory and copy invalid files to an output directory.

## Overview

`wav-files-validate` scans a specified input directory (including subdirectories) for `.wav` files, validates their format and integrity, and copies only the invalid files to a designated output directory. Valid files are skipped, and a summary report is printed to the console.

This tool helps isolate corrupted or malformed WAV files for further inspection or repair, ideal for audio archiving, data cleanup, or quality control in media pipelines.

## Features

- **Recursive Scanning**: Processes all subdirectories in the input folder.
- **WAV Integrity Validation**: Checks file headers, chunk structure, and audio data integrity using the `hound` crate.
- **Invalid File Isolation**: Copies only invalid files to the output directory, preserving the original relative path structure.
- **Progress Reporting**: Displays status for each file and a final summary of valid and invalid counts.
- **Dry Run Mode**: Simulates the process without copying files for testing.
- **Robust Error Handling**: Continues processing on errors, logging issues without halting.

## Installation

### Pre-built Binary (Recommended)

Download the latest release from the [GitHub Releases page](https://github.com/RustedBytes/wav-files-validate/releases). Extract the binary and add it to your PATH.

### From Source

1. Install Rust via [rustup](https://rustup.rs/).
2. Clone the repository:
   ```
   git clone https://github.com/RustedBytes/wav-files-validate.git
   cd wav-files-validate
   ```
3. Build and install:
   ```
   cargo install --path .
   ```

## Usage

```
wav-files-validate [OPTIONS] <INPUT_DIR> <OUTPUT_INVALID_DIR>
```

### Arguments

- `<INPUT_DIR>`: The input directory containing WAV files to validate (recursive).
- `<OUTPUT_INVALID_DIR>`: The output directory where invalid WAV files will be copied.

### Options

- `-h, --help`: Print help.
- `-v, --version`: Print version.
- `--dry-run`: Simulate validation and copying without actually copying files.

## Examples

Validate WAV files in `./audio/input` and copy invalid ones to `./audio/invalids`:

```
wav-files-validate ./audio/input ./audio/invalids
```

Run a dry run to preview invalid files that would be copied:

```
wav-files-validate --dry-run ./audio/input ./audio/invalids
```

## Building from Source

Ensure you have Rust installed, then:

```
git clone https://github.com/RustedBytes/wav-files-validate.git
cd wav-files-validate
cargo build --release
```

The binary will be in `target/release/wav-files-validate`.

## Dependencies

- `clap`: For command-line argument parsing.
- `hound`: For reading and validating WAV files.
- `walkdir`: For recursive directory traversal.

See `Cargo.toml` for versions.

## Testing

Run the test suite:

```
cargo test
```

Tests cover validation logic, recursive traversal, error handling, and edge cases like empty directories or non-WAV files.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on submitting issues or pull requests.

## Acknowledgments

- Built with Rust's excellent ecosystem.
- Thanks to the `hound` crate maintainers for robust WAV handling.
