//! Lilypond struct definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::lily::{
    LilyClef, LilyKey, LilyNoteLength, LilyNoteName, LilyTime, OctaveRelative, ToLily,
};

#[derive(Debug, Clone)]
pub struct Lilypond {
    pub version: String,
    pub staves: Vec<LilyStave>,
}

impl ToLily for Lilypond {
    fn to_lily(&self) -> String {
        let staves: String = self.staves.iter().map(|s| s.to_lily()).collect();

        format!("\\version \"{}\"\n<<{}>>", self.version, staves)
    }
}

#[derive(Debug, Clone)]
pub struct LilyStave {
    pub symbols: Vec<LilySymbol>,
}

impl ToLily for LilyStave {
    fn to_lily(&self) -> String {
        let symbols = self
            .symbols
            .iter()
            .map(|s| s.to_lily())
            .collect::<Vec<_>>()
            .join(" ");

        format!("\n\\new Staff {{ {} }}\n", symbols)
    }
}

#[derive(Debug, Clone)]
pub enum LilySymbol {
    Clef(LilyClef),
    Key(LilyKey),
    Time(LilyTime),
    Note(LilyNote),
}

impl ToLily for LilySymbol {
    fn to_lily(&self) -> String {
        match self {
            LilySymbol::Clef(sym) => sym.to_lily(),
            LilySymbol::Key(sym) => sym.to_lily(),
            LilySymbol::Time(sym) => sym.to_lily(),
            LilySymbol::Note(sym) => sym.to_lily(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LilyNote {
    pub note_name: LilyNoteName,
    pub octave_relative: OctaveRelative,
    pub length: LilyNoteLength,
}

impl ToLily for LilyNote {
    fn to_lily(&self) -> String {
        format!(
            "{}{}{}",
            self.note_name.to_lily(),
            self.octave_relative.to_lily(),
            self.length.to_lily()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::lily::LilyKeyType;

    use super::*;

    #[test]
    fn to_lily_note() {
        assert_eq!(
            LilyNote {
                note_name: LilyNoteName::Des,
                octave_relative: OctaveRelative::Up(2),
                length: LilyNoteLength::L8,
            }
            .to_lily(),
            "des''8"
        );

        assert_eq!(
            LilyNote {
                note_name: LilyNoteName::C,
                octave_relative: OctaveRelative::Down(1),
                length: LilyNoteLength::L1,
            }
            .to_lily(),
            "c,"
        );
    }

    #[test]
    fn to_lily_symbol() {
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
            LilySymbol::Clef(LilyClef::Treble).to_lily(),
            LilyClef::Treble.to_lily()
        );
        assert_eq!(
            LilySymbol::Key(key_c_major.clone()).to_lily(),
            key_c_major.to_lily()
        );
        assert_eq!(LilySymbol::Time(LilyTime::c()).to_lily(), "\\time 4/4");
        assert_eq!(
            LilySymbol::Note(note_c1_4.clone()).to_lily(),
            note_c1_4.to_lily()
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
    fn to_lily_stave() {
        // Arrange
        let (d_maj, d_maj_str) = stave_d_maj();

        // Act && Assert
        assert_eq!(d_maj.to_lily().trim(), d_maj_str);
    }

    #[test]
    fn to_lily_lilypond() {
        // Arrange
        let (d_maj, _) = stave_d_maj();
        let lilypond = Lilypond {
            version: "1.2.3".to_owned(),
            staves: vec![d_maj.clone(), d_maj.clone()],
        };

        // Act && Assert
        assert_eq!(
            lilypond.to_lily().trim(),
            format!(
                "\\version \"1.2.3\"\n<<{}{}>>",
                d_maj.to_lily(),
                d_maj.to_lily()
            )
        );
    }
}
