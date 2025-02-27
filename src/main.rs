#![windows_subsystem = "windows"]
// Import necessary modules
mod utils;
use utils::{list_archive_contents};
use eframe::egui;
use rfd::FileDialog;
use std::error::Error;

struct MyApp {
    archive_path: String,
    extract_to_path: String,
    password: String,
    message: String,
    archive_contents: Vec<String>, // New field to store archive contents

}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            archive_path: String::new(),
            extract_to_path: String::new(),
            password: String::new(),
            message: String::new(),
            archive_contents: Vec::new(), // Initialize as empty
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

                    // List archive contents
                    match list_archive_contents(&self.archive_path, None) {
                        Ok(contents) => {
                            self.archive_contents = contents;
                            self.message = "Archive contents loaded successfully!".to_string();
                        }
                        Err(e) => {
                            self.message = format!("Error listing archive contents: {}", e);
                            self.archive_contents.clear();
                        }
                    }
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
                    match crate::utils::extract_archive(
                        &self.archive_path,
                        &self.extract_to_path,
                        if self.password.is_empty() { None } else { Some(&self.password) },
                    ) {
                        Ok(_) => self.message = "Extraction successful!".to_string(),
                        Err(e) => self.message = format!("Error: {}", e),
                    };
                }
            }

            // Display Archive Contents (Scrollable)
            ui.separator();
            ui.heading("Archive Contents");

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());

                    if self.archive_contents.is_empty() {
                        ui.label("No contents to display.");
                    } else {
                        for item in &self.archive_contents {
                            ui.label(item);
                        }
                    }

            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "FerrisUnzip",
        options,
        Box::new(|_cc| {
            let app: Box<dyn eframe::App> = Box::new(MyApp::default());
            Ok(app)
        }),
    );
}