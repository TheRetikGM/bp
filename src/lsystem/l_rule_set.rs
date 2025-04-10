use std::{fmt::Display, rc::Rc};

use crate::lsystem::l_rule::{CSSLRule, LRule};
use crate::{error::Result, lsystem::l_rule::ToCSSLRule};

/// Set of all rules in a given L-System
pub trait LRuleSet<R: LRule>: Display {
    /// Select a matching rule from the set.
    fn select(&self, left: &str) -> Option<&Rc<R>>;

    fn rules(&self) -> &Vec<Rc<R>>;
}

/// Rule set of Context-Sensitive Stochastic L-System
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CSSLRuleSet {
    rules: Vec<Rc<CSSLRule>>,
}

impl CSSLRuleSet {
    pub fn new(rules: Vec<CSSLRule>) -> Self {
        Self {
            rules: rules.into_iter().map(Rc::new).collect(),
        }
    }

    pub fn from_str_rules(rules: &[&str]) -> Result<Self> {
        let mut rules_parsed: Vec<Rc<CSSLRule>> = vec![];
        rules_parsed.reserve_exact(rules.len());

        for r in rules.iter() {
            rules_parsed.push(Rc::new(r.to_csslrule()?))
        }

        Ok(Self {
            rules: rules_parsed,
        })
    }

    pub fn css_rules(&self) -> &Vec<Rc<CSSLRule>> {
        &self.rules
    }
}

impl LRuleSet<CSSLRule> for CSSLRuleSet {
    /// Select rule by taking all matching rules, scaling the random 0..1 value
    /// to be in range 0..<sum of all rule probabilities> and selecting random
    /// rule from that.
    fn select(&self, left: &str) -> Option<&Rc<CSSLRule>> {
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

    fn rules(&self) -> &Vec<Rc<CSSLRule>> {
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
