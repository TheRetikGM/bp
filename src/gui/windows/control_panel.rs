//! Control panel window definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::{
    gui::{gui_app::GuiAppState, windows::DockableWindow},
    lsystem::{CSSLSystem, LSystem},
};

#[derive(Debug)]
pub struct ControlPanel {
    n_steps: usize,
    prev_word: String,
}

impl ControlPanel {
    pub fn new(app_state: &GuiAppState) -> Self {
        Self {
            n_steps: 1,
            prev_word: app_state.axiom.clone(),
        }
    }

    pub fn step(&mut self, app_state: &mut GuiAppState, n: usize) {
        self.prev_word = app_state.l_system.state().word().clone();
        (0..n).for_each(|_| {
            app_state.used_rules_history.push(app_state.l_system.step());
        });
    }

    pub fn back(&mut self, app_state: &mut GuiAppState) {
        let iter_num = *app_state.l_system.state().iter_num();
        app_state
            .l_system
            .state_mut()
            .set_word(self.prev_word.clone())
            .set_iter_num(iter_num - 1);
    }

    pub fn retry_step(&mut self, app_state: &mut GuiAppState) {
        self.back(app_state);
        self.step(app_state, 1);
        app_state.dirty = true;
    }
}

impl DockableWindow for ControlPanel {
    fn name(&self) -> &'static str {
        "Control panel"
    }

    fn show(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) {
        ui.vertical(|ui| {
            if ui.button("Step").clicked() {
                self.step(app_state, 1);
            }

            if ui.button("RetryStep").clicked() {
                self.retry_step(app_state);
            }

            if ui.button("Back").clicked() {
                self.back(app_state);
            }

            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut self.n_steps).speed(1.0));
                if ui.button("Steps").clicked() {
                    self.step(app_state, self.n_steps);
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    app_state.l_system =
                        CSSLSystem::new(app_state.axiom.clone(), app_state.rules.clone());
                    app_state.dirty = true;
                }

                if ui.button("Refresh").clicked() {
                    app_state.dirty = true;
                }
            });
        });
    }
}
