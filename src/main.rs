//! Main entry point to application.
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

pub struct ControlPanel;
use music_sheet_gen::{
    gui::{GuiApp, DIR_NAME},
    Arguments,
};

// When compiling natively:
fn main() -> eframe::Result {
    if let Err(e) = Arguments::new() {
        eprintln!("error: {e}\n");
        Arguments::help();

        return Ok(());
    }

    egui_logger::builder()
        .max_level(log::LevelFilter::Debug)
        .init()
        .expect("Failed to initialize egui_logger");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        persistence_path: directories::BaseDirs::new()
            .map(|base_dirs| base_dirs.data_dir().join(DIR_NAME).join("data.ron")),
        ..Default::default()
    };
    eframe::run_native(
        "Music sheet generation",
        native_options,
        Box::new(|cc| Ok(Box::new(GuiApp::new(cc)))),
    )
}
