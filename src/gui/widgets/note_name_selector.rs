//! Note name selector widget definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use egui::Align;

use crate::notation::{Accidental, ExtNoteName, NoteName};

fn get_grid(id: egui::Id, num_columns: usize) -> egui::Grid {
    let grid_spacing = 4.0;

    egui::Grid::new(id)
        .num_columns(num_columns)
        .striped(false)
        .min_col_width(0.0)
        .spacing([grid_spacing, grid_spacing])
}

pub struct NoteNameSelector<'a> {
    ext: &'a mut ExtNoteName,
}

impl<'a> NoteNameSelector<'a> {
    pub fn new(ext: &'a mut ExtNoteName) -> Self {
        Self { ext }
    }

    fn show_selector(&mut self, ui: &mut egui::Ui) {
        let note_name = &mut self.ext.note_name;
        let acc = &mut self.ext.accidental;

        ui.with_layout(egui::Layout::left_to_right(Align::TOP), |ui| {
            let frame_nn = egui::Frame::dark_canvas(ui.style());

            frame_nn.show(ui, |ui| {
                get_grid(ui.unique_id(), 3).show(ui, |ui| {
                    ui.selectable_value(note_name, NoteName::C, "C");
                    ui.selectable_value(note_name, NoteName::D, "D");
                    ui.selectable_value(note_name, NoteName::E, "E");
                    ui.end_row();
                    ui.selectable_value(note_name, NoteName::F, "F");
                    ui.selectable_value(note_name, NoteName::G, "G");
                    ui.selectable_value(note_name, NoteName::A, "A");
                    ui.end_row();
                    ui.selectable_value(note_name, NoteName::B, "B");
                    ui.end_row();
                });
            });

            let frame_acc = egui::Frame::dark_canvas(ui.style());

            frame_acc.show(ui, |ui| {
                get_grid(ui.unique_id(), 1).show(ui, |ui| {
                    ui.selectable_value(acc, None, "-");
                    ui.end_row();

                    ui.selectable_value(acc, Some(Accidental::Sharp), "#");
                    ui.end_row();

                    ui.selectable_value(acc, Some(Accidental::Flat), "b");
                    ui.end_row();
                });
            });
        });
    }
}

impl<'a> egui::Widget for NoteNameSelector<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::new()
            .show(ui, |ui| {
                self.show_selector(ui);
            })
            .response
    }
}
