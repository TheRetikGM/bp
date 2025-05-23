//! L-system rewriter definitions
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

pub struct ControlPanel;
use std::rc::Rc;

use crate::lsystem::{
    l_rule::LRule,
    l_rule_set::{CSSLRuleSet, LRuleSet},
    CSSLRule,
};

/// Generic rewriter using the given ruleset.
pub trait LRewriter<R: LRule, S: LRuleSet<R>> {
    fn rules(&self) -> &S;
    fn max_lside_len(&self) -> i32;

    /// Rewrite the given L-system string and return the new string and used rules.
    fn rewrite(&self, s: &str) -> (String, Vec<Rc<R>>) {
        // We start from the right (right derivation).
        let mut i = s.len() as i32 - 1;
        // Store used left sides of rules for future new string construction.
        let mut res: Vec<&str> = Vec::new();
        let mut used_rules: Vec<Rc<R>> = vec![];

        loop {
            // Get the left side to search in rules. We move the window rtl.
            let pot = i - self.max_lside_len() + 1;
            let j = if pot < 0 { 0 } else { pot };
            let str_view = &s[j as usize..=i as usize];

            // Select rule, which matches the selected left side.
            let (left, right) = match self.rules().select(str_view) {
                Some(r) => {
                    used_rules.push(r.clone());
                    (r.left(), r.right())
                }
                None => ("_", &str_view[str_view.len() - 1..]),
            };

            // Store the right side for future reconstruction.
            res.push(right);

            // Move the window to left. We need to advance by length of the
            // replaced left side, so we don't interlace the replacements.
            i -= left.len() as i32;
            if i < 0 {
                break;
            }
        }

        // As the rules were added from the right to left, we need to
        // concat them in reverse order.
        res.reverse();
        (res.join(""), used_rules)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CSSLRewriter {
    rules: CSSLRuleSet,
}

impl Default for CSSLRewriter {
    fn default() -> Self {
        Self {
            rules: CSSLRuleSet::new(vec![]),
        }
    }
}

impl CSSLRewriter {
    pub fn new(rules: CSSLRuleSet) -> Self {
        Self { rules }
    }
}

impl LRewriter<CSSLRule, CSSLRuleSet> for CSSLRewriter {
    fn rules(&self) -> &CSSLRuleSet {
        &self.rules
    }

    fn max_lside_len(&self) -> i32 {
        3
    }
}
