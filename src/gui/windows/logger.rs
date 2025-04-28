//! Logger window definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::gui::{windows::DockableWindow, GuiAppState};

#[derive(Debug)]
pub struct Logger;

impl DockableWindow for Logger {
    fn name(&self) -> &'static str {
        "Logger"
    }

    fn show(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) {
        egui_logger::logger_ui().show(ui);
    }
}
