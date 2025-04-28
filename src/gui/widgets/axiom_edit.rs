//! Axiom edit widget definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use egui::{Color32, FontId, RichText};

use crate::gui::View;

#[derive(Debug, Default)]
pub struct AxiomEdit;

impl View for AxiomEdit {
    fn ui(&mut self, ui: &mut egui::Ui, app_state: &mut crate::gui::GuiAppState) {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new("TODO: axiom edit")
                    .color(Color32::RED)
                    .font(FontId::proportional(24.0)),
            );
        });
    }
}
