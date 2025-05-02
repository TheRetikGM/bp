//! Rule sums widget definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use egui::{Color32, RichText};
use itertools::Itertools;

use crate::{gui::widgets::RULE_EPS, lsystem::CSSLRule};

#[derive(Debug)]
pub struct RuleSums<'a> {
    rules: &'a Vec<CSSLRule>,
}

impl<'a> RuleSums<'a> {
    pub fn new(rules: &'a Vec<CSSLRule>) -> Self {
        Self { rules }
    }

    fn make_grid_contents(&mut self, ui: &mut egui::Ui) {
        let rules_map = self
            .rules
            .iter()
            .map(|r| (r.left().chars().last().unwrap(), r.p()))
            .into_group_map();

        ui.strong("Context char");
        ui.strong("Probability sum");
        ui.strong("Diff");
        ui.end_row();

        for (char, rules) in rules_map.iter().sorted_by(|(&a, _), (&b, _)| a.cmp(&b)) {
            let p_sum: f32 = rules.iter().copied().sum();

            let sum_color = if (1.0 - RULE_EPS..=1.0 + RULE_EPS).contains(&p_sum) {
                Color32::GREEN
            } else {
                Color32::RED
            };

            ui.label(RichText::new(char.to_string()).color(Color32::ORANGE));
            ui.label(RichText::new(format!("{p_sum:.4}")).color(sum_color));
            ui.label(format!("{0:e}", 1.0 - p_sum));
            ui.end_row();
        }
    }
}

impl<'a> egui::Widget for RuleSums<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::new()
            .show(ui, |ui| {
                egui::Grid::new("rule_sums")
                    .num_columns(3)
                    .striped(true)
                    .spacing([20.0, 4.0])
                    .show(ui, |ui| self.make_grid_contents(ui));
            })
            .response
    }
}
