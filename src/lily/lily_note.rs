//! Lilypond note definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::{
    lily::{LilyNoteLength, LilyNoteName, OctaveRelative},
    notation::Note,
};

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

impl std::fmt::Display for LilyNote {
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
    use super::*;

    #[test]
    fn to_string() {
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
            "c,1"
        );
    }
}
