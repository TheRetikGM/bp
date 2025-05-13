//! Statistics window definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::{
    gui::{utils, windows::DockableWindow, GuiAppState, TabType},
    lsystem::LSystem,
};

#[derive(Debug, Default)]
pub struct Statistics;

impl Statistics {
    pub fn make_grid_contents(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) {
        ui.strong("Left");
        ui.label("");
        ui.strong("Right");
        ui.label("");
        ui.strong("Probability");
        ui.end_row();

        for r in app_state.used_rules_history.last().unwrap_or(&vec![]) {
            ui.label(r.left());
            ui.label("->");
            ui.label(r.right());
            ui.label("%");
            ui.label(format!("{:.2}", r.p()));
            ui.end_row();
        }
    }
}

impl DockableWindow for Statistics {
    fn name(&self) -> &'static str {
        TabType::Statistics.as_str()
    }

    fn show(&mut self, ui: &mut egui::Ui, app_state: &mut crate::gui::GuiAppState) {
        utils::section_name(ui, "Current Word");
        ui.group(|ui| {
            ui.set_max_height(50.0);
            ui.add(egui::Label::new(app_state.l_system.state().word()).wrap());
        });

        utils::section_name(ui, "Last step used rules");
        ui.horizontal(|ui| {
            ui.add_space(25.0);
            egui::Grid::new("rule_sums")
                .num_columns(3)
                .striped(true)
                .spacing([20.0, 4.0])
                .show(ui, |ui| self.make_grid_contents(ui, app_state));
        });
    }
}
