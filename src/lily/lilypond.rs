//! Lilypond struct definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use std::fmt::Display;

use crate::{
    lily::{LilyClef, LilyKey, LilyNoteLength, LilyNoteName, LilyTime, OctaveRelative},
    notation::{Note, Score, Stave, Symbol},
};

#[derive(Debug, Clone)]
pub struct Lilypond {
    pub version: String,
    pub staves: Vec<LilyStave>,
}

impl Default for Lilypond {
    fn default() -> Self {
        Self {
            version: "2.25.20".to_string(),
            staves: Default::default(),
        }
    }
}

impl Display for Lilypond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let staves: String = self.staves.iter().map(|s| s.to_string()).collect();

        write!(f, "\\version \"{}\"\n<<{}>>", self.version, staves)
    }
}

impl From<Score> for Lilypond {
    fn from(value: Score) -> Self {
        Self {
            staves: value.tracks.into_iter().map(Into::into).collect(),
            version: "2.25.2".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LilyStave {
    pub symbols: Vec<LilySymbol>,
}

impl Display for LilyStave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbols = self
            .symbols
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "\n\\new Staff {{ {} }}\n", symbols)
    }
}

impl From<Stave> for LilyStave {
    fn from(stave: Stave) -> Self {
        Self {
            symbols: stave.symbols.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LilySymbol {
    Clef(LilyClef),
    Key(LilyKey),
    Time(LilyTime),
    Note(LilyNote),
}

impl From<Symbol> for LilySymbol {
    fn from(sym: Symbol) -> Self {
        match sym {
            Symbol::Clef(clef) => LilySymbol::Clef(clef.into()),
            Symbol::TimeSignature(time_signature) => LilySymbol::Time(time_signature.into()),
            Symbol::KeySignature(key_signature) => LilySymbol::Key(key_signature.into()),
            Symbol::Note(note) => LilySymbol::Note(note.into()),
            _ => todo!("Not implemented yet"),
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct LilyNote {
    pub note_name: LilyNoteName,
    pub octave_relative: OctaveRelative,
    pub length: LilyNoteLength,
}

impl From<Note> for LilyNote {
    fn from(note: Note) -> Self {
        let octave = note.pitch.octave;

        Self {
            note_name: note.pitch.into(),
            length: note.duration.into(),
            octave_relative: octave.into(),
        }
    }
}

impl Display for LilyNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.note_name, self.octave_relative, self.length
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::lily::LilyKeyType;

    use super::*;

    #[test]
    fn to_string_note() {
        assert_eq!(
            LilyNote {
                note_name: LilyNoteName::Des,
                octave_relative: OctaveRelative::Up(2),
                length: LilyNoteLength::L8,
            }
            .to_string(),
            "des''8"
        );

        assert_eq!(
            LilyNote {
                note_name: LilyNoteName::C,
                octave_relative: OctaveRelative::Down(1),
                length: LilyNoteLength::L1,
            }
            .to_string(),
            "c,"
        );
    }

    #[test]
    fn to_string_symbol() {
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

    fn stave_d_maj() -> (LilyStave, &'static str) {
        use LilyNoteName::*;

        let d_maj_def = [
            LilySymbol::Clef(LilyClef::Treble),
            LilySymbol::Key(LilyKey {
                note: D,
                key_type: LilyKeyType::Major,
            }),
        ]
        .iter();
        let d_maj_notes = [D, E, Fis, G, A, H, C].iter().map(|n| {
            LilySymbol::Note(LilyNote {
                note_name: n.clone(),
                octave_relative: OctaveRelative::Up(1),
                length: LilyNoteLength::L4,
            })
        });
        let d_maj = LilyStave {
            symbols: d_maj_def.cloned().chain(d_maj_notes).collect(),
        };

        (
            d_maj,
            "\\new Staff { \\clef treble \\key d \\major d'4 e'4 fis'4 g'4 a'4 h'4 c'4 }",
        )
    }

    #[test]
    fn to_string_stave() {
        // Arrange
        let (d_maj, d_maj_str) = stave_d_maj();

        // Act && Assert
        assert_eq!(d_maj.to_string().trim(), d_maj_str);
    }

    #[test]
    fn to_string_lilypond() {
        // Arrange
        let (d_maj, _) = stave_d_maj();
        let lilypond = Lilypond {
            version: "1.2.3".to_owned(),
            staves: vec![d_maj.clone(), d_maj.clone()],
        };

        // Act && Assert
        assert_eq!(
            lilypond.to_string().trim(),
            format!("\\version \"1.2.3\"\n<<{}{}>>", d_maj, d_maj)
        );
    }
}
