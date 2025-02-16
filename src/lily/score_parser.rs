//! Implementation of score parser

use crate::notation::{visitor::*, *};

pub struct ScoreParser {}

impl SheetVisitor for ScoreParser {
    fn visit_score(&mut self, score: &Score) {
        todo!()
    }

    fn visit_track(&mut self, score: &Stave) {
        todo!()
    }

    fn visit_symbol(&mut self, score: &Symbol) {
        todo!()
    }

    fn order(&self) -> &VisitOrder {
        &VisitOrder::Pre
    }
}
