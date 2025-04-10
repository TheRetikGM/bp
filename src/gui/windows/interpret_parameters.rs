//! Interpreter parameters window definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::{
    gui::{
        gui_app::GuiAppState,
        utils,
        widgets::{LengthSelector, NoteNameSelector, OctaveSelector},
        windows::DockableWindow,
    },
    lsystem::interpret::ScaleType,
    notation::KeySignatureType,
};

#[derive(Debug, Default)]
pub struct InterpretParameteres;

impl InterpretParameteres {
    fn show_grid_contents(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) {
        let info = &mut app_state.music_int_info;

        utils::section_name(ui, "Key");

        ui.label("Clef");
        ui.horizontal(|ui| {
            let clef = &mut info.clef;
            ui.selectable_value(clef, crate::notation::Clef::Treble, "Treble");
            ui.selectable_value(clef, crate::notation::Clef::Bass, "Bass");
        });
        ui.end_row();

        ui.label("Note");
        ui.add(NoteNameSelector::new(&mut info.key_signature.ext));
        ui.end_row();

        ui.label("Type");
        ui.horizontal(|ui| {
            let key_type = &mut info.key_signature.signature_type;
            ui.selectable_value(key_type, KeySignatureType::Maj, "Major");
            ui.selectable_value(key_type, KeySignatureType::Min, "Minor");
        });
        ui.end_row();

        ui.label("Scale type");
        ui.horizontal(|ui| {
            let scale_type = &mut info.scale_type;
            ui.selectable_value(scale_type, ScaleType::Basic, "Basic");
            ui.selectable_value(scale_type, ScaleType::JazzLike, "Jazz-like");
        });
        ui.end_row();

        utils::section_name(ui, "First note");

        ui.label("Pitch note");
        ui.add(NoteNameSelector::new(&mut info.first_note.pitch.ext));
        ui.end_row();

        ui.label("Pitch octave");
        ui.add(OctaveSelector::new(&mut info.first_note.pitch.octave));
        ui.end_row();

        ui.label("Length");
        ui.add(LengthSelector::new(&mut info.first_note.duration));
        ui.end_row();

        utils::section_name(ui, "Time signature");

        ui.label("Nom");
        ui.add(egui::Slider::new(
            &mut info.time_signature.beat_count,
            2..=64,
        ));
        ui.end_row();

        ui.label("Denom");
        ui.add(LengthSelector::new(
            &mut info.time_signature.single_beat_note,
        ));
        ui.end_row();

        utils::section_name(ui, "Tempo");

        ui.label("Note length");
        ui.add(LengthSelector::new(&mut info.tempo.note_length));
        ui.end_row();

        ui.label("Speed");
        ui.add(egui::Slider::new(&mut info.tempo.speed, 60..=220));
        ui.end_row();

        utils::section_name(ui, "Lilypond sanitizer");

        ui.label("Max line bars");
        ui.add(egui::Slider::new(
            &mut app_state.lily_sanitizer.max_line_bars,
            1..=30,
        ));
        ui.end_row();

        ui.label("Max line notes");
        ui.add(egui::Slider::new(
            &mut app_state.lily_sanitizer.max_line_notes,
            1..=70,
        ));
        ui.end_row();
    }
}

impl DockableWindow for InterpretParameteres {
    fn name(&self) -> &'static str {
        "Interpret parameters"
    }

    fn show(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) {
        egui::Grid::new("grid:interpret_parameters")
            .striped(true)
            .spacing([40.0, 4.0])
            .num_columns(2)
            .show(ui, |ui| {
                self.show_grid_contents(ui, app_state);
            });
    }
}
