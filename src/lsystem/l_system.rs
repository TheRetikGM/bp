use derive_getters::Getters;

use crate::lsystem::{
    l_rewriter::{CSSLRewriter, LRewriter},
    l_rule::CSSLRule,
    l_rule_set::CSSLRuleSet,
    LRule, LRuleSet,
};
use std::{fmt::Display, rc::Rc};

#[derive(Debug, Getters, serde::Deserialize, serde::Serialize)]
pub struct LSystemState {
    iter_num: i32,
    word: String,
}

impl LSystemState {
    pub fn set_iter_num(&mut self, new_iter_num: i32) -> &mut Self {
        self.iter_num = new_iter_num;
        self
    }

    pub fn set_word(&mut self, new_word: String) -> &mut Self {
        self.word = new_word;
        self
    }
}

impl Display for LSystemState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(iter {}): {}", self.iter_num, self.word)
    }
}

pub trait LSystem<R: LRule, S: LRuleSet<R>, W: LRewriter<R, S>>: Display + std::fmt::Debug {
    /// Advance the L-System by rewriting the stored word.
    fn step(&mut self) -> Vec<Rc<R>> {
        let rewrite_result = self.rewriter().rewrite(self.state().word.as_ref());

        self.state_mut().word = rewrite_result.0;
        self.state_mut().iter_num += 1;

        rewrite_result.1
    }
    fn state(&self) -> &LSystemState;
    fn state_mut(&mut self) -> &mut LSystemState;
    fn rewriter(&self) -> &W;
    fn rewriter_mut(&mut self) -> &mut W;
}

/// Context-Sensitive Stochastic L-System
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CSSLSystem {
    rewriter: CSSLRewriter,

    axiom: String,
    state: LSystemState,
}

impl CSSLSystem {
    /// Create new CSSLSystem
    ///
    /// # Parameters
    /// - **axiom** Small string that will be expanded by rules
    /// - **rules** Stochastic context-sennsitive string rewriting rules
    pub fn from(axiom: String, rules: &[&str]) -> Self {
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

    pub fn new(axiom: String, ruleset: CSSLRuleSet) -> Self {
        Self {
            rewriter: CSSLRewriter::new(ruleset),
            axiom: axiom.clone(),
            state: LSystemState {
                word: axiom,
                iter_num: 0,
            },
        }
    }
}

impl LSystem<CSSLRule, CSSLRuleSet, CSSLRewriter> for CSSLSystem {
    fn state(&self) -> &LSystemState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut LSystemState {
        &mut self.state
    }

    fn rewriter(&self) -> &CSSLRewriter {
        &self.rewriter
    }
    fn rewriter_mut(&mut self) -> &mut CSSLRewriter {
        &mut self.rewriter
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
