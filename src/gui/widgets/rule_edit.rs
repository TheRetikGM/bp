//! Rule edit widget definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use std::{collections::HashMap, rc::Rc};

use egui::{Color32, RichText};
use egui_extras::{Column, TableBuilder};

use crate::{
    error::AppError,
    gui::View,
    lsystem::{l_rule::ToCSSLRule, CSSLRule, CSSLRuleSet},
};

#[derive(Debug, Default)]
pub struct RuleEdit {
    pub text: String,
    rules: Vec<Rc<CSSLRule>>,
    rules_map: HashMap<char, Vec<Rc<CSSLRule>>>,
}

const RULE_EPS: f32 = 0.001;

impl RuleEdit {
    pub fn import_rules(&mut self, rules: &CSSLRuleSet) {
        self.rules.clear();
        self.rules_map.clear();
        self.text.clear();

        for r in rules.css_rules().iter() {
            let last_char = r.left().chars().last().unwrap();
            let rc = Rc::new(r.clone());

            self.rules.push(rc.clone());

            if let Some(rules) = self.rules_map.get_mut(&last_char) {
                rules.push(rc);
            } else {
                self.rules_map.insert(last_char, vec![rc]);
            }

            self.text.push_str(r.to_string().as_str());
            self.text.push('\n');
        }
    }

    pub fn export_rules(&self) -> Vec<CSSLRule> {
        self.rules.iter().map(|r| r.as_ref().clone()).collect()
    }

    fn prob_sums(&mut self, ui: &mut egui::Ui, app_state: &mut crate::gui::GuiAppState) {
        let available_height = ui.available_height();
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().at_least(40.0))
            .column(Column::auto().at_least(40.0))
            .column(Column::auto().at_least(40.0))
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        let row_height = 18.0;
        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Context char");
                });
                header.col(|ui| {
                    ui.strong("Probability sum");
                });
                header.col(|ui| {
                    ui.strong("Diff");
                });
            })
            .body(|mut body| {
                for (char, rules) in &self.rules_map {
                    let p_sum: f32 = rules.iter().map(|r| *r.p()).sum();

                    let sum_color = if (1.0 - RULE_EPS..=1.0 + RULE_EPS).contains(&p_sum) {
                        Color32::GREEN
                    } else {
                        Color32::RED
                    };

                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(RichText::new(char.to_string()).color(Color32::ORANGE));
                        });
                        row.col(|ui| {
                            ui.label(RichText::new(p_sum.to_string()).color(sum_color));
                        });
                        row.col(|ui| {
                            ui.label((1.0 - p_sum).to_string());
                        });
                    });
                }
            });
    }

    fn update_rules(&mut self) {
        self.rules.clear();
        self.rules_map.clear();

        let non_empty_lines = self
            .text
            .lines()
            .map(|l| l.trim())
            .filter(|line| !line.is_empty());

        for line in non_empty_lines {
            let rule = Rc::new(if let Ok(r) = line.to_csslrule() {
                r
            } else {
                continue;
            });

            let last_char = rule.left().chars().last().unwrap();
            self.rules.push(rule.clone());
            if let Some(rules) = self.rules_map.get_mut(&last_char) {
                rules.push(rule);
            } else {
                self.rules_map.insert(last_char, vec![rule]);
            }
        }
    }

    pub fn check(&self) -> crate::error::Result<()> {
        let non_empty_lines = self
            .text
            .lines()
            .map(|l| l.trim())
            .filter(|line| !line.is_empty());

        for line in non_empty_lines {
            line.to_csslrule()?;
        }

        for (last_char, rules) in &self.rules_map {
            let p_sum: f32 = rules.iter().map(|r| *r.p()).sum();
            if !(1.0 - RULE_EPS..=1.0 + RULE_EPS).contains(&p_sum) {
                Err(AppError::CSSRuleSumNotOne(*last_char, RULE_EPS))?
            }
        }

        Ok(())
    }
}

impl View for RuleEdit {
    fn ui(&mut self, ui: &mut egui::Ui, app_state: &mut crate::gui::GuiAppState) {
        let text = &mut self.text;

        let output = egui::TextEdit::multiline(text)
            .hint_text("Rule format: abc -> def % 1/2")
            .show(ui);

        if output.response.changed() {
            self.update_rules();
        }

        ui.separator();

        ui.collapsing("Sum of probabilities", |ui| {
            self.prob_sums(ui, app_state);
        });
    }
}
