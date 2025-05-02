//! Octave selector widget definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::notation::Octave;

pub struct OctaveSelector<'a> {
    octave: &'a mut Octave,
}

impl<'a> OctaveSelector<'a> {
    pub fn new(octave: &'a mut Octave) -> Self {
        Self { octave }
    }

    fn show_abs(&mut self, ui: &mut egui::Ui) {
        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            use Octave::*;
            for (i, &o) in [O0, O1, O2, O3, O4, O5, O6, O7, O8].iter().enumerate() {
                ui.selectable_value(self.octave, o, i.to_string());
            }
        });
    }

    fn show_rel(&mut self, ui: &mut egui::Ui) {
        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            use Octave::*;

            for (i, &o) in [O0, O1, O2, O3, O4, O5, O6, O7, O8].iter().enumerate() {
                ui.selectable_value(self.octave, o, (i as i32 - 3).to_string());
            }
        });
    }
}

impl<'a> egui::Widget for OctaveSelector<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        egui::Grid::new(ui.unique_id())
            .num_columns(2)
            .spacing([10.0, 4.0])
            .min_col_width(0.0)
            .show(ui, |ui| {
                ui.label("Abs");
                self.show_abs(ui);
                ui.end_row();

                ui.label("Rel");
                self.show_rel(ui);
            })
            .response
    }
}
