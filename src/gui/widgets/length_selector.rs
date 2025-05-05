//! Length selector widget definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::notation::NoteLength;

pub struct LengthSelector<'a> {
    note_length: &'a mut NoteLength,
}

impl<'a> LengthSelector<'a> {
    pub fn new(note_length: &'a mut NoteLength) -> Self {
        Self { note_length }
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            egui::Grid::new(ui.unique_id())
                .num_columns(8)
                .min_col_width(0.0)
                .show(ui, |ui| {
                    use NoteLength::*;
                    for (i, &l) in [L1, L2, L4, L8, L16, L32, L64, L128].iter().enumerate() {
                        ui.selectable_value(self.note_length, l, (1 << i).to_string());
                    }
                });
        });
    }
}

impl<'a> egui::Widget for LengthSelector<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::new()
            .show(ui, |ui| {
                self.show(ui);
            })
            .response
    }
}
