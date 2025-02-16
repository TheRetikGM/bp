//! Visitor pattern for the music notation
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::notation::{score::Score, stave::Stave, symbol::Symbol};

pub enum VisitOrder {
    Pre,
    Post,
}

pub trait SheetVisitor {
    fn visit_score(&mut self, score: &Score);
    fn visit_track(&mut self, score: &Stave);
    fn visit_symbol(&mut self, score: &Symbol);

    fn order(&self) -> &VisitOrder;
}

pub trait SheetVisitable {
    fn accept(&self, visitor: &mut impl SheetVisitor);
}
