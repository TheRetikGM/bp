//! Track definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::notation::{symbol::Symbol, visitor::SheetVisitable};

#[derive(Debug, Default)]
pub struct Track {
    pub symbols: Vec<Symbol>,
}

impl SheetVisitable for Track {
    fn accept(&self, visitor: &mut impl super::visitor::SheetVisitor) {
        match visitor.order() {
            super::visitor::VisitOrder::Pre => {
                visitor.visit_track(self);
                self.symbols.iter().for_each(|s| s.accept(visitor));
            }
            super::visitor::VisitOrder::Post => {
                self.symbols.iter().for_each(|s| s.accept(visitor));
                visitor.visit_track(self);
            }
        }
    }
}
