use eframe::egui;
use rfd::FileDialog;
use zip::ZipArchive;
use sevenz_rust::{decompress_file_with_password, Password};
use tar::Archive as TarArchive;
use flate2::read::GzDecoder;
use bzip2::read::BzDecoder;
use xz2::read::XzDecoder;
use std::error::Error;
use std::ffi::CString;
use std::fs::{self, File};
use std::io;
use unrar::{Archive, OpenArchive, UnrarResult};

use std::path::{Path, PathBuf};


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
            "rar" => ArchiveType::Rar,
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
fn extract_7z(archive: &str, extract_to: &str, password: Option<&str>) -> Result<(), Box<dyn Error>> {
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

// Extract RAR archive
fn extract_rar(archive_path: &str, extract_to: &str, password: Option<&str>) -> Result<(), Box<dyn Error>> {
    // Create the RAR archive with or without a password
    let mut archive = match password {
        Some(pwd) => Archive::with_password(archive_path, pwd).open_for_processing()?,
        None => Archive::new(archive_path).open_for_processing()?,
    };

    // Ensure the extraction directory exists
    fs::create_dir_all(extract_to)?;

    // Process each file in the archive
    while let Some(header) = archive.read_header()? {
        let dest_path = Path::new(extract_to).join(header.entry().filename.to_string_lossy().as_ref());

        if header.entry().is_directory() {
            // Create directories
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
        ArchiveType::Rar => extract_rar(archive, extract_to, password),
        ArchiveType::Unknown => Err("Unsupported archive format".into()),
    }
}

// Egui Application State
struct MyApp {
    archive_path: String,
    extract_to_path: String,
    password: String,
    message: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            archive_path: String::new(),
            extract_to_path: String::new(),
            password: String::new(),
            message: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FerrisUnzip");

            // Select Archive File
            if ui.button("Select Archive").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Archives", &["zip", "7z", "tar", "gz", "bz2", "xz", "rar"])
                    .pick_file()
                {
                    self.archive_path = path.to_string_lossy().to_string();
                }
            }
            ui.label(format!("Archive Path: {}", self.archive_path));

            // Select Extract Directory
            if ui.button("Select Extract Directory").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.extract_to_path = path.to_string_lossy().to_string();
                }
            }
            ui.label(format!("Extract To: {}", self.extract_to_path));

            // Password Input
            ui.label("Password:");
            ui.text_edit_singleline(&mut self.password);

            // Extract Button
            if ui.button("Extract").clicked() {
                if self.archive_path.is_empty() || self.extract_to_path.is_empty() {
                    self.message = "Please select both archive and extract directory.".to_string();
                } else {
                    match extract_archive(
                        &self.archive_path,
                        &self.extract_to_path,
                        if self.password.is_empty() { None } else { Some(&self.password) },
                    ) {
                        Ok(_) => self.message = "Extraction successful!".to_string(),
                        Err(e) => self.message = format!("Error: {}", e),
                    };
                }
            }

            // Display Message
            ui.label(&self.message);
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "FerrisUnzip",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
}