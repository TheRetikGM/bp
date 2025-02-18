//! Track definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::notation::symbol::Symbol;

#[derive(Debug, Default, Clone)]
pub struct Stave {
    pub symbols: Vec<Symbol>,
}
