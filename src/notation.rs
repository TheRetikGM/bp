//! Module which defines the stored structure of a score
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

mod score;
mod stave;
mod symbol;

pub use score::Score;
pub use score::ScoreInfo;
pub use stave::Stave;
pub use symbol::Chord;
pub use symbol::Note;
pub use symbol::Pitch;
pub use symbol::Symbol;

#[rustfmt::skip] #[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Clef { Treble, Bass }

#[rustfmt::skip] #[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum KeySignatureType { Maj, Min }

#[rustfmt::skip] #[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NoteName { C, D, E, F, G, A, B }

#[rustfmt::skip] #[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Octave { O0, O1, O2, O3, O4, O5, O6, O7, O8, O9 }

#[rustfmt::skip] #[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Accidental { Sharp, Flat }

#[rustfmt::skip] #[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Duration { D1, D2, D4, D8, D16, D32, D64, D128 }

// FIXME: Add support for accidentals
#[derive(Debug, Clone, Copy)]
pub struct KeySignature {
    pub note: NoteName,
    pub signature_type: KeySignatureType,
}

#[derive(Debug, Clone, Copy)]
pub struct TimeSignature {
    pub beat_count: u8,
    pub single_beat_note: Duration,
}

impl NoteName {
    pub fn next(&self) -> Self {
        match self {
            NoteName::C => NoteName::D,
            NoteName::D => NoteName::E,
            NoteName::E => NoteName::F,
            NoteName::F => NoteName::G,
            NoteName::G => NoteName::A,
            NoteName::A => NoteName::B,
            NoteName::B => NoteName::C,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            NoteName::C => NoteName::B,
            NoteName::D => NoteName::C,
            NoteName::E => NoteName::D,
            NoteName::F => NoteName::E,
            NoteName::G => NoteName::F,
            NoteName::A => NoteName::G,
            NoteName::B => NoteName::A,
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            NoteName::C => 0,
            NoteName::D => 1,
            NoteName::E => 2,
            NoteName::F => 3,
            NoteName::G => 4,
            NoteName::A => 5,
            NoteName::B => 6,
        }
    }

    pub fn value_halftone(&self) -> u8 {
        match self {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => 9,
            NoteName::B => 11,
        }
    }

    pub fn tone_count() -> u8 {
        7
    }

    pub fn halftone_count() -> u8 {
        12
    }
}

impl Octave {
    pub fn try_next(&self) -> Option<Self> {
        match self {
            Octave::O0 => Some(Octave::O1),
            Octave::O1 => Some(Octave::O2),
            Octave::O2 => Some(Octave::O3),
            Octave::O3 => Some(Octave::O4),
            Octave::O4 => Some(Octave::O5),
            Octave::O5 => Some(Octave::O6),
            Octave::O6 => Some(Octave::O7),
            Octave::O7 => Some(Octave::O8),
            Octave::O8 => Some(Octave::O9),
            Octave::O9 => None,
        }
    }

    pub fn try_prev(&self) -> Option<Self> {
        match self {
            Octave::O0 => None,
            Octave::O1 => Some(Octave::O0),
            Octave::O2 => Some(Octave::O1),
            Octave::O3 => Some(Octave::O2),
            Octave::O4 => Some(Octave::O3),
            Octave::O5 => Some(Octave::O4),
            Octave::O6 => Some(Octave::O5),
            Octave::O7 => Some(Octave::O6),
            Octave::O8 => Some(Octave::O7),
            Octave::O9 => Some(Octave::O8),
        }
    }

    fn value(&self) -> u8 {
        match self {
            Octave::O0 => 0,
            Octave::O1 => 1,
            Octave::O2 => 2,
            Octave::O3 => 3,
            Octave::O4 => 4,
            Octave::O5 => 5,
            Octave::O6 => 6,
            Octave::O7 => 7,
            Octave::O8 => 8,
            Octave::O9 => 9,
        }
    }
}

impl Duration {
    pub fn half(&self) -> Option<Self> {
        match self {
            Duration::D1 => None,
            Duration::D2 => Some(Duration::D1),
            Duration::D4 => Some(Duration::D2),
            Duration::D8 => Some(Duration::D4),
            Duration::D16 => Some(Duration::D8),
            Duration::D32 => Some(Duration::D16),
            Duration::D64 => Some(Duration::D32),
            Duration::D128 => Some(Duration::D64),
        }
    }
}

impl TimeSignature {
    pub fn c() -> Self {
        Self {
            beat_count: 4,
            single_beat_note: Duration::D4,
        }
    }
}
