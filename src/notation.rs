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
pub enum NoteLength { L1, L2, L4, L8, L16, L32, L64, L128 }

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ExtNoteName {
    pub note_name: NoteName,
    pub accidental: Option<Accidental>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeySignature {
    pub ext: ExtNoteName,
    pub signature_type: KeySignatureType,
}

#[derive(Debug, Clone, Copy)]
pub struct TimeSignature {
    pub beat_count: u8,
    pub single_beat_note: NoteLength,
}

#[derive(Debug, Clone, Copy)]
pub struct Tempo {
    pub note_length: NoteLength,
    pub speed: u8,
}

impl Default for Tempo {
    fn default() -> Self {
        Self {
            note_length: NoteLength::L4,
            speed: 100,
        }
    }
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

    /// Number of tones in one octave
    pub fn tone_count() -> u8 {
        7
    }

    // Number of halftones in one octave
    pub fn halftone_count() -> u8 {
        12
    }
}

impl NoteLength {
    pub fn halve(&mut self) {
        match self {
            NoteLength::L1 => *self = NoteLength::L2,
            NoteLength::L2 => *self = NoteLength::L4,
            NoteLength::L4 => *self = NoteLength::L8,
            NoteLength::L8 => *self = NoteLength::L16,
            NoteLength::L16 => *self = NoteLength::L32,
            NoteLength::L32 => *self = NoteLength::L64,
            NoteLength::L64 => *self = NoteLength::L128,
            NoteLength::L128 => {
                eprintln!("WARNING: Trying to halve 1/128 note. Ignoring.");
            }
        }
    }

    pub fn half(&self) -> Option<Self> {
        match self {
            NoteLength::L1 => Some(NoteLength::L2),
            NoteLength::L2 => Some(NoteLength::L4),
            NoteLength::L4 => Some(NoteLength::L8),
            NoteLength::L8 => Some(NoteLength::L16),
            NoteLength::L16 => Some(NoteLength::L32),
            NoteLength::L32 => Some(NoteLength::L64),
            NoteLength::L64 => Some(NoteLength::L128),
            NoteLength::L128 => None,
        }
    }
}

impl TimeSignature {
    pub fn c() -> Self {
        Self {
            beat_count: 4,
            single_beat_note: NoteLength::L4,
        }
    }
}
