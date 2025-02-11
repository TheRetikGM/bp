//! Struct definitions
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::notation::{track::Track, visitor::SheetVisitable};

type Tempo = u32;

#[derive(Debug)]
pub struct ScoreInfo {
    pub name: Option<String>,
    pub author: Option<String>,
    pub transcriber: Option<String>,
}

#[derive(Debug)]
pub struct Score {
    pub tracks: Vec<Track>,
    pub info: ScoreInfo,
    pub tempo: Tempo,
}

impl SheetVisitable for Score {
    fn accept(&self, visitor: &mut impl super::visitor::SheetVisitor) {
        match visitor.order() {
            super::visitor::VisitOrder::Pre => {
                visitor.visit_score(self);
                self.tracks.iter().for_each(|t| t.accept(visitor));
            }
            super::visitor::VisitOrder::Post => {
                self.tracks.iter().for_each(|t| t.accept(visitor));
                visitor.visit_score(self);
            }
        }
    }
}
