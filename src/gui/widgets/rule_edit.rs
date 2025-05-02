//! Rule edit widget definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use std::f32;

use itertools::Itertools;

use crate::{
    error::AppError,
    gui::widgets::RULE_EPS,
    lsystem::{l_rule::ToCSSLRule, CSSLRule, CSSLRuleSet},
};

#[derive(Debug, Default)]
pub struct RuleEditState {
    pub text: String,
    pub rules: Vec<CSSLRule>,
}

impl RuleEditState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rules(mut self, ruleset: &CSSLRuleSet) -> Self {
        self.text.clear();
        self.rules.clear();

        for r in ruleset.css_rules().iter() {
            self.text.push_str(r.to_string().as_str());
            self.text.push('\n');
            self.rules.push(r.clone());
        }

        self
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

        let rules_map = self
            .rules
            .iter()
            .map(|r| (r.left().chars().last().unwrap(), r.p()))
            .into_group_map();

        for (last_char, rules) in rules_map {
            let p_sum: f32 = rules.into_iter().sum();
            if !(1.0 - super::RULE_EPS..=1.0 + super::RULE_EPS).contains(&p_sum) {
                Err(AppError::CSSRuleSumNotOne(last_char, RULE_EPS))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct RuleEdit<'a> {
    pub state: &'a mut RuleEditState,
}

impl<'a> RuleEdit<'a> {
    pub fn new(state: &'a mut RuleEditState) -> Self {
        Self { state }
    }

    fn update_rules(&mut self) {
        self.state.rules.clear();

        let non_empty_lines = self
            .state
            .text
            .lines()
            .map(|l| l.trim())
            .filter(|line| !line.is_empty());

        for line in non_empty_lines {
            let rule = if let Ok(r) = line.to_csslrule() {
                r
            } else {
                continue;
            };

            self.state.rules.push(rule.clone());
        }
    }

    pub fn show(mut self, ui: &mut egui::Ui) {
        let text = &mut self.state.text;

        let output = egui::TextEdit::multiline(text)
            .hint_text("Rule format: abc -> def % 1/2")
            .desired_width(f32::INFINITY)
            .show(ui);
        output.text_clip_rect.height();

        if output.response.changed() {
            self.update_rules();
        }
    }
}
