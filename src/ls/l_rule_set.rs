use std::fmt::Display;

use crate::ls::l_rule::{CSSLRule, LRule};

/// Set of all rules in a given L-System
pub trait LRuleSet: Display {
    /// Select a matching rule from the set.
    fn select(&self, left: &str) -> Option<&impl LRule>;

    fn rules(&self) -> &Vec<impl LRule>;
}

/// Rule set of Context-Sensitive Stochastic L-System
#[derive(Debug, Clone)]
pub struct CSSLRuleSet {
    rules: Vec<CSSLRule>,
}

impl CSSLRuleSet {
    pub fn new(rules: Vec<CSSLRule>) -> Self {
        Self { rules }
    }
}

impl LRuleSet for CSSLRuleSet {
    /// Select rule by taking all matching rules, scaling the random 0..1 value
    /// to be in range 0..<sum of all rule probabilities> and selecting random
    /// rule from that.
    fn select(&self, left: &str) -> Option<&impl LRule> {
        let matching_rules: Vec<_> = self.rules.iter().filter(|r| r.matches(left)).collect();

        let total_p: f32 = matching_rules.iter().map(|r| r.p()).sum();
        let rnd = rand::random::<f32>() * total_p;

        let mut acc = 0.;
        matching_rules
            .iter()
            .find(|r| {
                acc += r.p();
                acc > rnd
            })
            .copied()
    }

    fn rules(&self) -> &Vec<impl LRule> {
        &self.rules
    }
}

impl Display for CSSLRuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ {} }}",
            self.rules
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
