//! Lilypond symbol deinition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use std::fmt::Display;

use crate::{
    lily::{lily_note::LilyNote, LilyClef, LilyKey, LilyTempo, LilyTime},
    notation::Symbol,
};

#[derive(Debug, Clone)]
pub enum LilySymbol {
    Clef(LilyClef),
    Key(LilyKey),
    Time(LilyTime),
    Note(LilyNote),
    Tempo(LilyTempo),
    Break,
}

impl From<Symbol> for LilySymbol {
    fn from(sym: Symbol) -> Self {
        match sym {
            Symbol::Clef(clef) => LilySymbol::Clef(clef.into()),
            Symbol::TimeSignature(time_signature) => LilySymbol::Time(time_signature.into()),
            Symbol::KeySignature(key_signature) => LilySymbol::Key(key_signature.into()),
            Symbol::Note(note) => LilySymbol::Note(note.into()),
            Symbol::Tempo(tempo) => LilySymbol::Tempo(tempo.into()),
            _ => panic!("Invalid symbol for conversion"),
        }
    }
}

impl Display for LilySymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LilySymbol::Clef(sym) => sym.fmt(f),
            LilySymbol::Key(sym) => sym.fmt(f),
            LilySymbol::Time(sym) => sym.fmt(f),
            LilySymbol::Note(sym) => sym.fmt(f),
            LilySymbol::Tempo(sym) => sym.fmt(f),
            LilySymbol::Break => write!(f, "\\break"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lily::*;

    #[test]
    fn to_string() {
        // Arrange
        let key_c_major = LilyKey {
            note: LilyNoteName::C,
            key_type: LilyKeyType::Major,
        };
        let note_c1_4 = LilyNote {
            note_name: LilyNoteName::C,
            octave_relative: OctaveRelative::Up(1),
            length: LilyNoteLength::L4,
        };

        // Act && Assert
        assert_eq!(
            LilySymbol::Clef(LilyClef::Treble).to_string(),
            LilyClef::Treble.to_string()
        );
        assert_eq!(
            LilySymbol::Key(key_c_major.clone()).to_string(),
            key_c_major.to_string()
        );
        assert_eq!(LilySymbol::Time(LilyTime::c()).to_string(), "\\time 4/4");
        assert_eq!(
            LilySymbol::Note(note_c1_4.clone()).to_string(),
            note_c1_4.to_string()
        );
    }
}
