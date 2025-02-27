# FerrisUnzip

FerrisUnzip is a command-line tool written in Rust for extracting various archive formats. It supports ZIP, 7Z (with password protection), TAR, TAR.GZ, TAR.BZ2, TAR.XZ, GZ, BZ2, XZ, and RAR archives.

## Features

-   **Multi-format support:** Extracts ZIP, 7Z, TAR, TAR.GZ, TAR.BZ2, TAR.XZ, GZ, BZ2, XZ, and RAR archives.
-   **Password protection:** Supports password-protected 7Z archives.
-   **Cross-platform:** Built with Rust, making it cross-platform compatible.
-   **Interactive extraction directory prompt:** Allows users to specify the extraction destination.
-   **Automatic directory creation:** Creates necessary directories during extraction.
-   **Password retry:** Prompts for a password if an encrypted 7Z archive is detected without one.

## Prerequisites

-   Rust and Cargo installed.

## Installation

1.  Clone the repository:

    ```bash
    git clone https://github.com/th3l3gend2777/FerrisUnzip/
    cd FerrisUnzip
    ```

2.  Build the project:

    ```bash
    cargo build --release
    ```

3.  The executable will be located at `target/release/FerrisUnzip`. You can copy it to a directory in your PATH for easy access.

## Usage

```bash
FerrisUnzip <archive_file> [OPTIONS]
```

# Dependancies

    clap: For command-line argument parsing.
    zip: For ZIP archive extraction.
    sevenz-rust: For 7Z archive extraction.
    tar: For TAR archive extraction.
    flate2: For GZ decompression.
    bzip2: For BZ2 decompression.
    xz2: For XZ decompression.
    unrar: For RAR archive extraction.

# Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.
# License

This project is licensed under the GPL License.
