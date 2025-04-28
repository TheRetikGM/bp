//! Grammar edit window definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::{
    error::Result,
    gui::{gui_app::GuiAppState, toast, windows::DockableWindow, View},
    lsystem::CSSLRuleSet,
};

#[derive(Debug)]
pub struct GrammarEdit {
    rule_edit: crate::gui::widgets::RuleEdit,
    axiom_edit: crate::gui::widgets::AxiomEdit,
}

impl GrammarEdit {
    pub fn new(app_state: &GuiAppState) -> Self {
        let mut s = Self {
            rule_edit: Default::default(),
            axiom_edit: Default::default(),
        };

        s.rule_edit.import_rules(&app_state.rules);

        s
    }

    fn apply(&mut self, app_state: &mut GuiAppState) -> Result<()> {
        self.rule_edit.check()?;
        app_state.rules = CSSLRuleSet::new(self.rule_edit.export_rules());
        app_state.apply_changes()?;

        Ok(())
    }
}

impl DockableWindow for GrammarEdit {
    fn name(&self) -> &'static str {
        "Grammar edit"
    }

    fn show(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) {
        self.ui(ui, app_state);
    }
}

impl View for GrammarEdit {
    fn ui(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) {
        ui.collapsing("Rules", |ui| {
            self.rule_edit.ui(ui, app_state);
        });

        ui.collapsing("Axiom", |ui| {
            self.axiom_edit.ui(ui, app_state);
        });

        ui.separator();

        ui.vertical_centered(|ui| {
            if ui.button("Apply").clicked() {
                match self.apply(app_state) {
                    Ok(_) => {
                        log::info!("Grammar applied.");
                        toast::show_success("Grammar applied.")
                    }
                    Err(err) => {
                        log::error!("{err}");
                        toast::show_error(err.to_string().as_str())
                    }
                }
            }
        });
    }
}
