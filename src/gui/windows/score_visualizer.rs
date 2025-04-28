//! Score visualizer window definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::gui::{gui_app::GuiAppState, windows::DockableWindow};

#[derive(Debug)]
pub struct ScoreVisualizer;

impl DockableWindow for ScoreVisualizer {
    fn name(&self) -> &'static str {
        "Score visualizer"
    }

    fn show(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) {
        ui.label("todo");
    }
}
