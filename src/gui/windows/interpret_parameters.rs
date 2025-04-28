//! Interpreter parameters window definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::gui::{gui_app::GuiAppState, windows::DockableWindow};

#[derive(Debug)]
pub struct InterpretParameteres;

impl DockableWindow for InterpretParameteres {
    fn name(&self) -> &'static str {
        "Interpret parameters"
    }

    fn show(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) {
        ui.label("todo");
    }
}
