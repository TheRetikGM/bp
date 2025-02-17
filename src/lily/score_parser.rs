//! Implementation of score parser

use crate::{
    lily::Lilypond,
    notation::{visitor::*, *},
};

#[derive(Default)]
pub struct ScoreParser {
    pub lilypond: Lilypond,
}

impl SheetVisitor for ScoreParser {
    fn visit_score(&mut self, score: &Score) {
        // TODO: Add Name
        // TODO: Add author
        // TODO: Add Transcriber
        // TODO: Add tempo marks
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
