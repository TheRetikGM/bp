//! Visitor pattern for the music notation
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::notation::{score::Score, symbol::Symbol, track::Track};

pub enum VisitOrder {
    Pre,
    Post,
}

pub trait SheetVisitor {
    fn visit_score(&self, score: &Score);
    fn visit_track(&self, score: &Track);
    fn visit_symbol(&self, score: &Symbol);

    fn order(&self) -> &VisitOrder;
}

pub trait SheetVisitable {
    fn accept(&self, visitor: &mut impl SheetVisitor);
}
