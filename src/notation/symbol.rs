//! Symbol definitions
//!
//! ### Author
//! Jakub Kloub

use crate::notation::*;

#[derive(Debug, Clone)]
pub struct Pitch {
    pub note_name: NoteName,
    pub octave: Octave,
    pub accidental: Option<Accidental>,
}

#[derive(Debug)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: Duration,
}

/// Represents a chord of notes that have the same length.
#[derive(Debug)]
pub struct Chord {
    pub pitches: Vec<Pitch>,
    pub duration: Duration,
}

#[derive(Debug)]
pub enum Symbol {
    Clef(Clef),
    TimeSignature(TimeSignature),
    KeySignature(KeySignature),
    Chord(Chord),
    Note(Note),
    Rest(Duration),
}
