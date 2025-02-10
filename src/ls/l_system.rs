use derive_getters::Getters;

use crate::ls::{
    l_rewriter::{CSSLRewriter, LRewriter},
    l_rule::CSSLRule,
    l_rule_set::CSSLRuleSet,
};
use std::fmt::Display;

#[derive(Debug, Getters)]
pub struct LSystemState {
    iter_num: i32,
    word: String,
}

impl Display for LSystemState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(iter {}): {}", self.iter_num, self.word)
    }
}

pub trait LSystem: Display + std::fmt::Debug {
    /// Advance the L-System by rewriting the stored word.
    fn step(&mut self) {
        self.state_mut().word = self.rewriter().rewrite(self.state().word.as_ref());
        self.state_mut().iter_num += 1;
    }
    fn state(&self) -> &LSystemState;
    fn state_mut(&mut self) -> &mut LSystemState;
    fn rewriter(&self) -> &impl LRewriter;
}

#[derive(Debug)]
struct CSSLSystem {
    rewriter: CSSLRewriter,
    axiom: String,
    state: LSystemState,
}

impl CSSLSystem {
    fn new(axiom: String, rules: &[&str]) -> Self {
        Self {
            rewriter: CSSLRewriter::new(CSSLRuleSet::new(
                rules.iter().map(|s| CSSLRule::from(s).unwrap()).collect(),
            )),
            axiom: axiom.clone(),
            state: LSystemState {
                word: axiom,
                iter_num: 0,
            },
        }
    }
}

impl LSystem for CSSLSystem {
    fn state(&self) -> &LSystemState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut LSystemState {
        &mut self.state
    }

    fn rewriter(&self) -> &impl LRewriter {
        &self.rewriter
    }
}

impl Display for CSSLSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CSSLSystem: {{\n\taxiom = {}\n\trules = {}\n}}",
            self.axiom,
            self.rewriter.rules()
        )
    }
}
