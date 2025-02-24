use clap::{Arg, Command};
use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::path::{Path};
use zip::ZipArchive;
use sevenz_rust::{decompress_file_with_password, Password};
use tar::Archive as TarArchive;
use flate2::read::GzDecoder;
use bzip2::read::BzDecoder;
use xz2::read::XzDecoder;
use unrar::Archive;
use fltk::{
    app, button::Button, dialog::NativeFileChooser, frame::Frame, input::Input, prelude::*,
    window::Window,
};

// Enum to represent supported archive types
#[derive(Debug)]
enum ArchiveType {
    Zip,
    SevenZ,
    Tar,
    TarGz,
    TarBz2,
    TarXz,
    Gz,
    Bz2,
    Xz,
    Rar,
    Unknown,
}

// Determine archive type based on file extension
fn get_archive_type(path: &Path) -> ArchiveType {
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        match ext.to_lowercase().as_str() {
            "zip" => ArchiveType::Zip,
            "7z" => ArchiveType::SevenZ,
            "tar" => ArchiveType::Tar,
            "gz" => {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem.ends_with(".tar") {
                        ArchiveType::TarGz
                    } else {
                        ArchiveType::Gz
                    }
                } else {
                    ArchiveType::Unknown
                }
            }
            "bz2" => {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem.ends_with(".tar") {
                        ArchiveType::TarBz2
                    } else {
                        ArchiveType::Bz2
                    }
                } else {
                    ArchiveType::Unknown
                }
            }
            "xz" => {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem.ends_with(".tar") {
                        ArchiveType::TarXz
                    } else {
                        ArchiveType::Xz
                    }
                } else {
                    ArchiveType::Unknown
                }
            }
            "rar" => ArchiveType::Rar, // <-- Add RAR detection
            _ => ArchiveType::Unknown,
        }
    } else {
        ArchiveType::Unknown
    }
}


// Extract ZIP archive (non-encrypted)
fn extract_zip(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(extract_to).join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

// Extract 7Z archive (supports encryption with password)
fn extract_7z(archive: &str, extract_to: &str, password: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(archive);

    if let Some(pwd) = password {
        let password = Password::from(pwd); // Convert to Password
        decompress_file_with_password(path, extract_to, password)?;
    } else {
        decompress_file_with_password(path, extract_to, Password::from(""))?; // Empty password for no encryption
    }
    Ok(())
}

// Extract plain TAR archive
fn extract_tar(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let mut archive = TarArchive::new(file); // Explicitly using TarArchive
    archive.unpack(extract_to)?; // No more method not found error
    Ok(())
}


// Extract TAR archive with compression
fn extract_tar_compressed(extract_to: &str, decoder: impl io::Read) -> Result<(), Box<dyn Error>> {
    let mut archive = TarArchive::new(decoder);
    archive.unpack(extract_to)?;
    Ok(())
}

// Extract TAR.GZ archive
fn extract_tar_gz(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let decoder = GzDecoder::new(file);
    extract_tar_compressed(extract_to, decoder)
}

// Extract TAR.BZ2 archive
fn extract_tar_bz2(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let decoder = BzDecoder::new(file);
    extract_tar_compressed(extract_to, decoder)
}

// Extract TAR.XZ archive
fn extract_tar_xz(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let decoder = XzDecoder::new(file);
    extract_tar_compressed(extract_to, decoder)
}

// Decompress single-file GZ
fn decompress_gz(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let mut decoder = GzDecoder::new(file);
    let output_file = Path::new(extract_to).join(Path::new(archive).file_stem().ok_or("Invalid filename")?);
    let mut outfile = File::create(output_file)?;
    io::copy(&mut decoder, &mut outfile)?;
    Ok(())
}

// Decompress single-file BZ2
fn decompress_bz2(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let mut decoder = BzDecoder::new(file);
    let output_file = Path::new(extract_to).join(Path::new(archive).file_stem().ok_or("Invalid filename")?);
    let mut outfile = File::create(output_file)?;
    io::copy(&mut decoder, &mut outfile)?;
    Ok(())
}

// Decompress single-file XZ
fn decompress_xz(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let mut decoder = XzDecoder::new(file);
    let output_file = Path::new(extract_to).join(Path::new(archive).file_stem().ok_or("Invalid filename")?);
    let mut outfile = File::create(output_file)?;
    io::copy(&mut decoder, &mut outfile)?;
    Ok(())
}

// Main extraction function
fn extract_archive(archive: &str, extract_to: &str, password: Option<&str>) -> Result<(), Box<dyn Error>> {
    let path = Path::new(archive);
    if !path.exists() {
        return Err("Archive file does not exist".into());
    }

    let archive_type = get_archive_type(path);
    match archive_type {
        ArchiveType::Zip => extract_zip(archive, extract_to),
        ArchiveType::SevenZ => extract_7z(archive, extract_to, password),
        ArchiveType::Tar => extract_tar(archive, extract_to),
        ArchiveType::TarGz => extract_tar_gz(archive, extract_to),
        ArchiveType::TarBz2 => extract_tar_bz2(archive, extract_to),
        ArchiveType::TarXz => extract_tar_xz(archive, extract_to),
        ArchiveType::Gz => decompress_gz(archive, extract_to),
        ArchiveType::Bz2 => decompress_bz2(archive, extract_to),
        ArchiveType::Xz => decompress_xz(archive, extract_to),
        ArchiveType::Rar => extract_rar(archive, extract_to), // <-- Use extract_rar function here
        ArchiveType::Unknown => Err("Unsupported archive format".into()),
    }
}



// Command-line interface
// FLTK GUI Implementation
fn main() {
    // Initialize FLTK application
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400, 300)
        .center_screen()
        .with_label("FerrisUnzip");

    // Widgets
    let mut archive_button = Button::new(10, 10, 380, 30, "Select Archive");
    let mut archive_path_frame = Frame::default().with_pos(10, 50).with_size(380, 30);
    let mut extract_to_button = Button::new(10, 90, 380, 30, "Select Extract Directory");
    let mut extract_to_frame = Frame::default().with_pos(10, 130).with_size(380, 30);
    let mut password_input = Input::new(40, 170, 380, 30, "Pass:");
    let mut extract_button = Button::new(10, 210, 380, 30, "Extract");

    // Variables to store selected paths
    let mut archive_path = String::new();
    let mut extract_to_path = String::new();

    // Select Archive File
    archive_button.set_callback({
        let mut archive_path_frame = archive_path_frame.clone();
        move |_| {
            let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseFile);
            chooser.set_filter("*.{zip,7z,tar,gz,bz2,xz,rar}");
            chooser.show();
            if let Some(path) = chooser.filename().to_str() {
                archive_path = path.to_string();
                archive_path_frame.set_label(&archive_path);
            }
        }
    });

    // Select Extract Directory
    extract_to_button.set_callback({
        let mut extract_to_frame = extract_to_frame.clone();
        move |_| {
            let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseFile);
            chooser.show();
            if let Some(path) = chooser.filename().to_str() {
                extract_to_path = path.to_string();
                extract_to_frame.set_label(&extract_to_path);
            }
        }
    });

    // Extract Button
    extract_button.set_callback({
        let archive_path_frame = archive_path_frame.clone();
        let extract_to_frame = extract_to_frame.clone();
        let password_input = password_input.clone();
        move |_| {
            // Bind the labels to variables to extend their lifetime
            let archive_label = archive_path_frame.label(); // Add this line
            let extract_to_label = extract_to_frame.label(); // Add this line

            // Convert to &str using .as_str()
            let archive = archive_label.as_str(); // Modify this line
            let extract_to = extract_to_label.as_str(); // Modify this line

            let password = password_input.value();

            if archive.is_empty() || extract_to.is_empty() {
                fltk::dialog::alert_default("Please select both archive and extract directory.");
                return;
            }

            match extract_archive(archive, extract_to, if password.is_empty() { None } else { Some(&password) }) {
                Ok(_) => fltk::dialog::message_default("Extraction successful!"),
                Err(e) => fltk::dialog::alert_default(&format!("Error: {}", e)),
            };
        }
    });

    // Show the window and run the application
    wind.end();
    wind.show();
    app.run().unwrap();
}


fn extract_rar(archive_path: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let mut archive = Archive::new(archive_path).open_for_processing()?;

    // Ensure the extraction directory exists
    fs::create_dir_all(extract_to)?;

    while let Some(header) = archive.read_header()? {
        let dest_path = Path::new(extract_to).join(header.entry().filename.to_string_lossy().as_ref());

        if header.entry().is_directory() {
            fs::create_dir_all(&dest_path)?;
            archive = header.skip()?;
        } else {
            // Ensure parent directories exist
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            // Extract the file to the destination
            archive = header.extract_to(&dest_path)?;
        }
    }

    Ok(())
}