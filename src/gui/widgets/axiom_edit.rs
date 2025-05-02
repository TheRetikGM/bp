//! Axiom edit widget definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use std::f32;

use egui::{Color32, RichText};

#[derive(Debug)]
pub struct AxiomEdit<'a> {
    text: &'a mut String,
}

impl<'a> AxiomEdit<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self { text }
    }
}

impl<'a> egui::Widget for AxiomEdit<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let frame = egui::Frame::new();

        frame
            .show(ui, |ui| {
                let has_whitespace = self.text.chars().any(|c: char| c.is_whitespace());

                let mut t = egui::TextEdit::singleline(self.text)
                    .hint_text("Type axiom string here")
                    .desired_width(f32::INFINITY);
                if has_whitespace {
                    t = t.text_color(Color32::ORANGE);
                }
                ui.add(t);

                if has_whitespace {
                    ui.label(RichText::new("Whitespaces are ignored").color(Color32::ORANGE));
                }
            })
            .response
    }
}
