//! Grammar edit window definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::{
    error::Result,
    gui::{
        gui_app::GuiAppState,
        toast,
        widgets::{AxiomEdit, RuleEdit, RuleEditState, RuleSums},
        windows::DockableWindow,
        View,
    },
    lsystem::CSSLRuleSet,
};

#[derive(Debug)]
pub struct GrammarEdit {
    rule_edit_state: RuleEditState,
    axiom: String,
}

impl GrammarEdit {
    pub fn new(app_state: &GuiAppState) -> Self {
        Self {
            rule_edit_state: RuleEditState::new().with_rules(&app_state.rules),
            axiom: app_state.axiom.clone(),
        }
    }

    fn apply(&mut self, app_state: &mut GuiAppState) -> Result<()> {
        self.rule_edit_state.check()?;
        app_state.rules = CSSLRuleSet::new(self.rule_edit_state.rules.clone());
        app_state.apply_changes()?;
        app_state.axiom = self.axiom.clone();

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
        egui::Grid::new("grid:grammar_edit")
            .num_columns(2)
            .striped(true)
            .spacing([40.0, 16.0])
            .show(ui, |ui| {
                ui.label("Rules");
                RuleEdit::new(&mut self.rule_edit_state).show(ui);
                ui.end_row();

                ui.label("Probability sums");
                ui.vertical(|ui| {
                    ui.add(RuleSums::new(&self.rule_edit_state.rules));
                });
                ui.end_row();

                ui.label("Axiom");
                ui.vertical(|ui| {
                    ui.add(AxiomEdit::new(&mut self.axiom));
                });
                ui.end_row();
            });

        ui.separator();

        ui.vertical_centered(|ui| {
            if ui.button("Apply").clicked() {
                match self.apply(app_state) {
                    Ok(_) => toast::show_success("Grammar applied."),
                    Err(err) => toast::show_error(err.to_string().as_str()),
                }
            }
        });
    }
}
