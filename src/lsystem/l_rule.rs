//! L-system rule definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

pub struct ControlPanel;
use std::fmt::Display;

use crate::error::*;
use crate::ext::*;
use derive_getters::Getters;
use once_cell::sync::Lazy;
use regex::Regex;

pub trait LRule: Display + ToString {
    fn matches(&self, str: &str) -> bool;
    fn left(&self) -> &str;
    fn right(&self) -> &str;
}

/// Represents a Context-Sensitive Stochastic L-System Rule in the
/// form $abc -> w, where a,b \in \Sigma |union {\eps}, c \in \Sigma and w \in \Sigma^*$
#[derive(Debug, Getters, Clone, serde::Deserialize, serde::Serialize)]
pub struct CSSLRule {
    /// Left side of the rule
    left: String,
    /// Right side of the rule
    right: String,
    /// Probability of the rule
    p: f32,
}

impl Display for CSSLRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {} % {}", self.left, self.right, self.p)
    }
}

impl LRule for CSSLRule {
    /// Check if string matches the rule
    ///
    /// # EXAMPLES
    /// ```
    /// # use music_sheet_gen::lsystem::l_rule::*;
    /// let r = CSSLRule::new("ab", "cd", 1.);
    /// assert!(r.matches("ab"));
    /// assert!(r.matches("12345ab"));
    /// assert!(!r.matches("b"));
    /// assert!(!r.matches("ab1234b"));
    /// ```
    fn matches(&self, str: &str) -> bool {
        (str.len() >= self.left().len()) && (&str[str.len() - self.left().len()..] == self.left())
    }

    fn left(&self) -> &str {
        &self.left
    }

    fn right(&self) -> &str {
        &self.right
    }
}

impl CSSLRule {
    pub fn new(left: &str, right: &str, p: f32) -> Self {
        Self {
            left: left.to_owned(),
            right: right.to_owned(),
            p,
        }
    }

    /// Convert string notation of CssLRule to its instance.
    /// Rule is in the form: A -> B % a/b
    /// Rule cannot contain the -> or white spaces in left and right side
    /// Rule cannot contain the % character.
    ///
    /// # Examples
    /// ```
    /// # use music_sheet_gen::lsystem::l_rule::*;
    /// let r = CSSLRule::from("a->b%1/2").unwrap();
    /// assert!(r.left() == "a");
    /// assert!(r.right() == "b");
    /// assert!(*r.p() == 1./2.);
    /// ```
    /// ```
    /// # use music_sheet_gen::lsystem::l_rule::*;
    /// let r = CSSLRule::from("abc -> def % 1/4").unwrap();
    /// assert!(r.left() == "abc");
    /// assert!(r.right() == "def");
    /// assert!(*r.p() == 1./4.);
    /// ```
    /// ```
    /// # use music_sheet_gen::lsystem::l_rule::*;
    /// let r = CSSLRule::from("abc -> def % 0.125").unwrap();
    /// assert!(r.left() == "abc");
    /// assert!(r.right() == "def");
    /// assert!(*r.p() == 0.125);
    /// ```
    pub fn from(s: impl AsRef<str>) -> Result<Self> {
        static REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.*?)->(.*?)%(.*?)/(.*?)$").unwrap());
        static REG_FB: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(.*?)->(.*?)%([\d\.]*?)$").unwrap());

        let s_without_whitespaces = s.without_whitespaces();

        if let Some(captures) = REG.captures(s_without_whitespaces.as_ref()) {
            let left = captures.captured_str(1)?.to_string();
            let right = captures.captured_str(2)?.to_string();
            let nom: i32 = captures.captured_str(3)?.parse()?;
            let denom: i32 =
                captures
                    .captured_str(4)?
                    .parse()
                    .map_err(|err: std::num::ParseIntError| {
                        AppError::CSSRuleParseNum(s.as_ref().to_string(), err.to_string())
                    })?;

            Ok(CSSLRule {
                left,
                right,
                p: nom as f32 / denom as f32,
            })
        } else if let Some(captures) = REG_FB.captures(s_without_whitespaces.as_ref()) {
            let left = captures.captured_str(1)?.to_string();
            let right = captures.captured_str(2)?.to_string();
            let p: f32 =
                captures
                    .captured_str(3)?
                    .parse()
                    .map_err(|err: std::num::ParseFloatError| {
                        AppError::CSSRuleParseNum(s.as_ref().to_string(), err.to_string())
                    })?;

            Ok(CSSLRule { left, right, p })
        } else {
            Err(AppError::CSSRuleParse(s.as_ref().to_string()).into())
        }
    }
}

pub trait ToCSSLRule {
    fn to_csslrule(&self) -> Result<CSSLRule>;
}

impl<T: AsRef<str>> ToCSSLRule for T {
    /// Use CSSLRule::from() on the given string.
    fn to_csslrule(&self) -> Result<CSSLRule> {
        CSSLRule::from(self)
    }
}
