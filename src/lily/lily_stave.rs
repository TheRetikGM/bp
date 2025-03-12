//! Lilypond stave definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use std::fmt::Display;

use crate::{lily::lily_symbol::LilySymbol, notation::Stave};

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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::lily::*;

    pub fn stave_d_maj() -> (LilyStave, &'static str) {
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
    fn to_string() {
        // Arrange
        let (d_maj, d_maj_str) = stave_d_maj();

        // Act && Assert
        assert_eq!(d_maj.to_string().trim(), d_maj_str);
    }
}
