use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use sevenz_rust::{decompress_file_with_password, Password};
use sevenz_rust::{Archive as SevenZArchive, Password as SevenZPassword};
use std::error::Error;
use std::fs::File;
use std::{fs, io};
use tar::Archive as TarArchive;
use unrar::{Archive as RarArchive, Archive};
use xz2::read::XzDecoder;
use zip::ZipArchive;
use std::path::Path;

pub(crate) enum ArchiveType {
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

pub fn get_archive_type(path: &Path) -> ArchiveType {
    if path.extension().map_or(false, |ext| ext == "zip") {
        ArchiveType::Zip
    } else if path.extension().map_or(false, |ext| ext == "7z") {
        ArchiveType::SevenZ
    } else if path.extension().map_or(false, |ext| ext == "tar") {
        ArchiveType::Tar
    } else if path.extension().map_or(false, |ext| ext == "gz") {
        ArchiveType::Gz
    } else if path.extension().map_or(false, |ext| ext == "bz2") {
        ArchiveType::Bz2
    } else if path.extension().map_or(false, |ext| ext == "xz") {
        ArchiveType::Xz
    } else if path.extension().map_or(false, |ext| ext == "rar") {
        ArchiveType::Rar
    } else {
        ArchiveType::Unknown
    }
}

// Extract 7Z archive (supports encryption with password)
pub fn extract_7z(archive: &str, extract_to: &str, password: Option<&str>) -> Result<(), Box<dyn Error>> {
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
pub fn extract_tar(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let mut archive = TarArchive::new(file); // Explicitly using TarArchive
    archive.unpack(extract_to)?; // No more method not found error
    Ok(())
}

pub fn extract_zip(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
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
pub fn extract_rar(archive_path: &str, extract_to: &str, password: Option<&str>) -> Result<(), Box<dyn Error>> {
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

// Extract TAR archive with compression
pub fn extract_tar_compressed(extract_to: &str, decoder: impl io::Read) -> Result<(), Box<dyn Error>> {
    let mut archive = TarArchive::new(decoder);
    archive.unpack(extract_to)?;
    Ok(())
}

// Extract TAR.GZ archive
 pub fn extract_tar_gz(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let decoder = GzDecoder::new(file);
    extract_tar_compressed(extract_to, decoder)
}

// Extract TAR.BZ2 archive
pub fn extract_tar_bz2(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let decoder = BzDecoder::new(file);
    extract_tar_compressed(extract_to, decoder)
}

// Extract TAR.XZ archive
pub fn extract_tar_xz(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let decoder = XzDecoder::new(file);
    extract_tar_compressed(extract_to, decoder)
}

// Decompress single-file GZ
pub fn decompress_gz(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let mut decoder = GzDecoder::new(file);
    let output_file = Path::new(extract_to).join(Path::new(archive).file_stem().ok_or("Invalid filename")?);
    let mut outfile = File::create(output_file)?;
    io::copy(&mut decoder, &mut outfile)?;
    Ok(())
}

// Decompress single-file BZ2
pub fn decompress_bz2(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let mut decoder = BzDecoder::new(file);
    let output_file = Path::new(extract_to).join(Path::new(archive).file_stem().ok_or("Invalid filename")?);
    let mut outfile = File::create(output_file)?;
    io::copy(&mut decoder, &mut outfile)?;
    Ok(())
}

// Decompress single-file XZ
pub fn decompress_xz(archive: &str, extract_to: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(archive)?;
    let mut decoder = XzDecoder::new(file);
    let output_file = Path::new(extract_to).join(Path::new(archive).file_stem().ok_or("Invalid filename")?);
    let mut outfile = File::create(output_file)?;
    io::copy(&mut decoder, &mut outfile)?;
    Ok(())
}

pub fn list_archive_contents(archive: &str, password: Option<&str>) -> Result<Vec<String>, Box<dyn Error>> {
    let path = Path::new(archive);
    if !path.exists() {
        return Err("Archive file does not exist".into());
    }

    let archive_type = get_archive_type(path);
    match archive_type {
        ArchiveType::Zip => {
            let file = File::open(archive)?;
            let mut archive = ZipArchive::new(file)?;
            let contents: Vec<String> = (0..archive.len())
                .map(|i| {
                    let file = archive.by_index(i).unwrap();
                    file.name().to_string()
                })
                .collect();
            Ok(contents)
        }
        ArchiveType::SevenZ => {
            // Open the 7z archive with or without a password
            let archive = match password {
                Some(pwd) => SevenZArchive::open_with_password(archive, &SevenZPassword::from(pwd))?,
                None => SevenZArchive::open(archive)?,
            };

            // Extract filenames from the archive's files field
            let contents: Vec<String> = archive
                .files
                .iter()
                .map(|entry| entry.name().to_string())
                .collect();

            Ok(contents)
        }
        ArchiveType::Tar => {
            let file = File::open(archive)?;
            let mut archive = TarArchive::new(file);
            let contents: Vec<String> = archive
                .entries()?
                .map(|entry| entry.unwrap().path().unwrap().to_string_lossy().to_string())
                .collect();
            Ok(contents)
        }
        ArchiveType::TarGz => {
            let file = File::open(archive)?;
            let decoder = GzDecoder::new(file);
            let mut archive = TarArchive::new(decoder);
            let contents: Vec<String> = archive
                .entries()?
                .map(|entry| entry.unwrap().path().unwrap().to_string_lossy().to_string())
                .collect();
            Ok(contents)
        }
        ArchiveType::TarBz2 => {
            let file = File::open(archive)?;
            let decoder = BzDecoder::new(file);
            let mut archive = TarArchive::new(decoder);
            let contents: Vec<String> = archive
                .entries()?
                .map(|entry| entry.unwrap().path().unwrap().to_string_lossy().to_string())
                .collect();
            Ok(contents)
        }
        ArchiveType::TarXz => {
            let file = File::open(archive)?;
            let decoder = XzDecoder::new(file);
            let mut archive = TarArchive::new(decoder);
            let contents: Vec<String> = archive
                .entries()?
                .map(|entry| entry.unwrap().path().unwrap().to_string_lossy().to_string())
                .collect();
            Ok(contents)
        }
        ArchiveType::Rar => {
            let mut archive = RarArchive::new(archive).open_for_processing()?;
            let mut contents = Vec::new();

            while let Some(header) = archive.read_header()? {
                let filename = header.entry().filename.to_string_lossy().to_string();
                contents.push(filename);
                archive = header.skip()?;
            }

            Ok(contents)
        }
        _ => Err("Unsupported archive format for listing".into()),
    }
}

pub fn extract_archive(
    archive: &str,
    extract_to: &str,
    password: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
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