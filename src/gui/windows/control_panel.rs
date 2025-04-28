//! Control panel window definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::gui::{gui_app::GuiAppState, windows::DockableWindow};

#[derive(Debug)]
pub struct ControlPanel;

impl DockableWindow for ControlPanel {
    fn name(&self) -> &'static str {
        "Control panel"
    }

    fn show(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) {
        ui.label("todo");
    }
}
