use crate::lsystem::{
    l_rule::LRule,
    l_rule_set::{CSSLRuleSet, LRuleSet},
};

pub trait LRewriter {
    fn rules(&self) -> &impl LRuleSet;
    fn max_lside_len(&self) -> i32;

    fn rewrite(&self, s: &str) -> String {
        let mut i = s.len() as i32 - 1;
        let mut res: Vec<&str> = Vec::new();

        loop {
            // Get the left side to search in rules. We move the window rtl.
            let pot = i - self.max_lside_len() + 1;
            let j = if pot < 0 { 0 } else { pot };
            let str_view = &s[j as usize..=i as usize];

            // Select rule, which matches the selected left side.
            let rule = self.rules().select(str_view);
            let replace = rule
                .map(|s| s.right())
                .unwrap_or(&str_view[str_view.len() - 1..]);

            // Store the right side for future reconstruction.
            res.push(replace);

            // Move the window to left. We need to advance by length of the
            // replaced left side, so we don't interlace the replacements.
            i -= rule.map(|r| r.left().len()).unwrap_or(1) as i32;
            if i < 0 {
                break;
            }
        }

        res.reverse();
        res.join("")
    }
}

#[derive(Debug, Clone)]
pub struct CSSLRewriter {
    rules: CSSLRuleSet,
}

impl CSSLRewriter {
    pub fn new(rules: CSSLRuleSet) -> Self {
        Self { rules }
    }
}

impl LRewriter for CSSLRewriter {
    fn rules(&self) -> &impl LRuleSet {
        &self.rules
    }

    fn max_lside_len(&self) -> i32 {
        3
    }
}
