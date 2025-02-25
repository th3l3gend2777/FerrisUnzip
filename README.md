

# FerrisUnzip

FerrisUnzip is a lightweight, cross-platform archive extraction tool built using Rust. It supports extracting files from various archive formats, including ZIP, 7Z, TAR, GZ, BZ2, XZ, and RAR. The application provides both a command-line interface (CLI) and a graphical user interface (GUI) powered by Egui, making it accessible for both developers and end-users.


---

## Table of Contents

1. [Features](#features)
2. [Installation](#installation)
3. [Usage](#usage)
    - [Command-Line Interface (CLI)](#command-line-interface-cli)
    - [Graphical User Interface (GUI)](#graphical-user-interface-gui)
4. [Supported Archive Formats](#supported-archive-formats)
5. [Dependencies](#dependencies)
6. [Contributing](#contributing)
7. [License](#license)

---

## Features

- **Multi-format Support**: Extract files from ZIP, 7Z, TAR, GZ, BZ2, XZ, and RAR archives.
- **Password Handling**: Supports password-protected archives (e.g., encrypted 7Z and RAR files).
- **Cross-Platform**: Runs on Windows, macOS, and Linux.
- **User-Friendly GUI**: Powered by Egui, providing an intuitive interface for selecting archives and extraction directories.
- **Command-Line Interface**: Allows for scripting and automation of archive extraction tasks.

---

## Installation

### Prerequisites

- **Rust Toolchain**: Ensure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Cargo**: The Rust package manager is required to build and run the application.

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/th3l3gend2777/FerrisUnzip/
   cd ferrisunzip
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   ./target/release/ferrisunzip
   ```

### Precompiled Binaries

Precompiled binaries for Windows, macOS, and Linux are available in the [Releases](https://github.com/th3l3gend2777/FerrisUnzip/releases/tag/release) section of the repository.

---

## Usage

### Command-Line Interface (CLI)

To extract an archive using the CLI, run the following command:

```bash
ferrisunzip <archive_path> <extract_to_directory> [--password <password>]
```

#### Example:

Extract a password-protected 7Z archive:
```bash
ferrisunzip example.7z /path/to/extract --password mypassword
```

Extract a plain TAR.GZ archive:
```bash
ferrisunzip example.tar.gz /path/to/extract
```

### Graphical User Interface (GUI)

1. Launch the application.
2. Use the "Select Archive" button to choose the archive file you want to extract.
3. Use the "Select Extract Directory" button to specify the destination folder.
4. If the archive is password-protected, enter the password in the provided field.
5. Click the "Extract" button to begin extraction.

![GUI Workflow](https://github.com/th3l3gend2777/FerrisUnzip/tree/nightly/Pictures/FerrisUnzip.png)


---

## Supported Archive Formats

The following archive formats are supported:

| Format | Extensions       | Password Support |
|--------|------------------|------------------|
| ZIP    | `.zip`           | No               |
| 7Z     | `.7z`            | Yes              |
| TAR    | `.tar`           | No               |
| TAR.GZ | `.tar.gz`, `.tgz`| No               |
| TAR.BZ2| `.tar.bz2`       | No               |
| TAR.XZ | `.tar.xz`        | No               |
| GZ     | `.gz`            | No               |
| BZ2    | `.bz2`           | No               |
| XZ     | `.xz`            | No               |
| RAR    | `.rar`           | Yes              |

---

## Dependencies

This project relies on several Rust crates and libraries:

- `eframe` and `egui`: For building the graphical user interface [[1]].
- `rfd`: For file and directory selection dialogs in the GUI.
- `zip`, `sevenz-rust`, `tar`, `flate2`, `bzip2`, `xz2`, `unrar`: For handling various archive formats.
- `clap`: For parsing command-line arguments (if applicable).

Install dependencies using Cargo:
```bash
cargo build
```

---

## Contributing

We welcome contributions! Here's how you can help:

1. **Bug Reports**: Open an issue on GitHub if you encounter any problems.
2. **Feature Requests**: Suggest new features or improvements via GitHub issues.
3. **Code Contributions**: Fork the repository, make your changes, and submit a pull request.

Please ensure your contributions adhere to the project's coding standards and include appropriate tests.

---

## License

This project is licensed under the GPL License. See the [LICENSE](https://github.com/th3l3gend2777/FerrisUnzip/blob/main/Licence.md) file for details.

---

## Additional Notes

- **Error Handling**: The application provides clear error messages for unsupported formats, missing files, and incorrect passwords.
- **Performance**: Optimized for handling large archives efficiently.
- **Future Enhancements**:
    - Add support for additional archive formats (e.g., ZST).
    - Improve the GUI with advanced features like progress indicators.

For more information, refer to the [project documentation](https://github.com/th3l3gend2777/FerrisUnzip).

---

This `README.md` file is designed to be comprehensive yet concise, ensuring that users can quickly understand the purpose and functionality of your project while also having access to detailed instructions for installation and usage.