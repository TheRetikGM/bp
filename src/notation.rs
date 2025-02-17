//! Module which defines the stored structure of a score
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

mod score;
mod stave;
mod symbol;
pub mod visitor;

pub use score::Score;
pub use score::ScoreInfo;
pub use stave::Stave;
pub use symbol::Chord;
pub use symbol::Note;
pub use symbol::Pitch;
pub use symbol::Symbol;

#[rustfmt::skip] #[derive(Debug, Clone, Copy)]
pub enum Clef { Treble, Bass }

#[rustfmt::skip] #[derive(Debug, Clone, Copy)]
pub enum KeySignatureType { Maj, Min }

#[rustfmt::skip] #[derive(Debug, Clone, Copy)]
pub enum NoteName { C, D, E, F, G, A, B }

#[rustfmt::skip] #[derive(Debug, Clone, Copy)]
pub enum Octave { O0, O1, O2, O3, O4, O5, O6, O7, O8, O9 }

#[rustfmt::skip] #[derive(Debug, Clone, Copy)]
pub enum Accidental { Sharp, Flat }

#[rustfmt::skip] #[derive(Debug, Clone, Copy)]
pub enum Duration { D1, D2, D4, D8, D16, D32, D64, D128 }

// FIXME: Add support for accidentals
#[derive(Debug)]
pub struct KeySignature {
    pub note: NoteName,
    pub signature_type: KeySignatureType,
}

#[derive(Debug)]
pub struct TimeSignature {
    pub beat_count: u8,
    pub single_beat_note: Duration,
}
