Archiver

A versatile Rust command-line tool for extracting a wide range of archive formats, including ZIP, 7Z, TAR, and compressed file formats like GZ, BZ2, and XZ. This tool supports encrypted 7Z archives by allowing users to provide a password.

    Disclaimer: This project is currently experimental and may not be fully secure. Use at your own risk. It is recommended to only extract trusted archives, especially when dealing with encrypted files.

Features

    Supports extraction for the following formats:
        ZIP (.zip)
        7Z (.7z)
        TAR (.tar)
        TAR.GZ (.tar.gz)
        TAR.BZ2 (.tar.bz2)
        TAR.XZ (.tar.xz)
        GZ (.gz)
        BZ2 (.bz2)
        XZ (.xz)
    Decompresses single-file GZ, BZ2, and XZ archives.
    Supports encrypted 7Z archives using a password.
    Extracts all file types to a specified directory.

Installation

You can build and install the project by following these steps:

    Clone the repository:git clone https://github.com/dragonfly939/Archiver.git
    cd Archiver



Build the project using Cargo:

    cargo build --release

    Run the tool:

    After building, the executable can be found in the target/release folder.

Usage

The extractor tool is a command-line utility that allows you to extract various archive formats by specifying the archive file and the destination directory.
Basic Usage

To extract an archive, simply run:

    ./target/release/extractor --archive <path_to_archive> --extract_to <destination_directory>

Extract a ZIP file:

    ./target/release/extractor --archive archive.zip --extract_to ./extracted_files

Extract a 7Z file (with password):

    ./target/release/extractor --archive archive.7z --extract_to ./extracted_files --password your_password

Extract a TAR.GZ file:

    ./target/release/extractor --archive archive.tar.gz --extract_to ./extracted_files

Extract a plain TAR file:

    ./target/release/extractor --archive archive.tar --extract_to ./extracted_files

Supported Formats

    ZIP (.zip)
    7Z (.7z)
    TAR (.tar)
    TAR.GZ (.tar.gz)
    TAR.BZ2 (.tar.bz2)
    TAR.XZ (.tar.xz)
    GZ (.gz)
    BZ2 (.bz2)
    XZ (.xz)

Options

    -p, --password <password>: Password for encrypted 7Z files. If the 7Z archive is not encrypted, this option can be omitted.
    --archive <path_to_archive>: Path to the archive file you wish to extract.
    --extract_to <destination_directory>: Directory where the files will be extracted.

Example

To extract an encrypted 7Z archive:

./target/release/extractor --archive archive.7z --extract_to ./extracted_files --password mysecurepassword

Error Handling

    Invalid Archive Format: The tool will inform you if an unsupported archive type is encountered.
    File Not Found: If the specified archive file does not exist, an error message will be displayed.
    Password Protection: If a password is required for a 7Z archive, but not provided, you will receive an error.

Dependencies

    clap for command-line argument parsing.
    zip for handling ZIP archives.
    sevenz_rust for handling 7Z archives.
    tar for handling TAR archives.
    flate2 for handling GZ compression.
    bzip2 for handling BZ2 compression.
    xz2 for handling XZ compression.

License

This project is licensed under the MIT License - see the LICENSE file for details.

Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for any bugs, suggestions, or enhancements.
